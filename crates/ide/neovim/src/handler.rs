use crate::diagnostic::{Diagnostic, DiagnosticType};
use crate::highlight::NodeHighlight;
use crate::span_ext::{LineCols, TextRangeExt};
use crate::state::State;
use crate::{Message, Neovim};
use anyhow::{bail, Result};
use async_trait::async_trait;
use futures::StreamExt;
use itertools::Itertools;
use neu_canceled::Canceled;
use neu_db::Diagnostician;
use neu_eval::db::Evaluator;
use neu_parser::{Arena, Node};
use neu_syntax::db::{FileKind, Parser};
use neu_syntax::Nodes;
use nvim_rs::rpc::IntoVal;
use nvim_rs::{compat::tokio::Compat, Handler};
use rmpv::Value;
use salsa::Database;
use std::future::Future;
use std::sync::Arc;
use tokio::io::Stdout;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct NeovimHandler {
    state: Arc<RwLock<Option<State>>>,
    tx: Arc<crossbeam::channel::Sender<Message>>,
}

impl NeovimHandler {
    pub fn new(message_tx: crossbeam::channel::Sender<Message>) -> Self {
        Self {
            state: Default::default(),
            tx: Arc::new(message_tx),
        }
    }
}

macro_rules! dbg {
    ($dbg_buffer: expr, $val:expr) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                writeln!($dbg_buffer, "[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp)?;
                tmp
            }
        }
    };
    // Trailing comma with single argument is ignored
    ($dbg_buffer: expr, $val:expr,) => { dbg!($dbg_buffer, $val) };
    ($dbg_buffer: expr, $($val:expr),+ $(,)?) => {
        ($(dbg!($dbg_buffer, $val)),+,)
    };
}

impl NeovimHandler {
    async fn handle_err(api: &Neovim, e: impl Future<Output = Result<()>>) {
        match e.await {
            Ok(_) => (),
            Err(e) => {
                log::error!("{:?}", &e);
                api.err_writeln(&format!("{:?}", e))
                    .await
                    .expect("Couldn't send error");
            }
        }
    }

    async fn on_load(&self, _args: Vec<Value>, api: &Neovim) -> Result<()> {
        let buf = api.get_current_buf().await?;

        if self.state.read().await.is_none() {
            //api.command("vsp").await?;
            //api.command("wincmd l").await?;

            let debug_bf = api.create_buf(true, true).await?;
            debug_bf.set_name("**NeuLang Debug**").await?;
            let win = api.get_current_win().await?;
            win.set_buf(&debug_bf).await?;

            let highlight_ns = api.create_namespace("NeuLang Highlight").await?;

            *self.state.write().await = Some(State::new(debug_bf, highlight_ns));

            api.command("sp").await?;
            api.command("bp").await?;
            //api.command("cope").await?;
            api.command(r#"echo "NeuLang Loaded""#).await?;
        }

        buf.attach(true, vec![]).await?;

        Ok(())
    }

    async fn on_nvim_buf_lines_event(&self, args: Vec<Value>, api: &Neovim) -> Result<()> {
        match &args[..] {
            [_cbf, _tick, Value::Integer(_first_line), Value::Integer(_last_line), Value::Array(_changed), Value::Boolean(_more)] =>
            {
                use std::fmt::Write;
                let mut dbg_buffer = String::new();

                let State {
                    debug_bf,
                    highlight_ns,
                } = self.state.read().await.clone().expect("State");

                let current_bf = api.get_current_buf().await?;
                let current_name = current_bf.get_name().await?;
                dbg!(dbg_buffer, &current_name);
                let file_id = (current_name, FileKind::Md);

                let my_tx = self.tx.clone();
                {
                    let lines = current_bf.get_lines(0, -1, false).await?;
                    let buf = lines.iter().join("\n");
                    if let Err(e) = my_tx.send(Message::Modified(file_id.clone(), buf)) {
                        eprintln!("{}", e);
                    }
                }
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                if let Err(e) = my_tx.send(Message::GetSnapshot(tx)) {
                    eprintln!("{}", e);
                }
                let db = match rx.next().await {
                    Some(db) => db,
                    None => {
                        return Ok(());
                    }
                };
                Canceled::cancel_if(db.salsa_runtime());
                let file_id = db.file_id(file_id);

                let buf = db.input(file_id);
                let lines = buf.lines().map(|l| l.to_string()).collect::<Vec<_>>();

                //let rendered = db.render_md(file_id.clone());

                let result = db.parse_syntax(file_id);

                writeln!(&mut dbg_buffer, "{}\n\n", result.display(&buf))?;

                let root = result.root;

                current_bf.clear_namespace(highlight_ns, 0, -1).await?;

                let _eval_result = db.eval(file_id, root);
                let arena: &Arena = &result.arena;

                // Highlighting

                let futures = arena
                    .iter()
                    .rev()
                    .filter_map(|node: &Node| node.highlight().map(|hl| (node, hl)))
                    .flat_map(|(node, hl)| {
                        node.span
                            .lines_cols(&lines)
                            .into_iter()
                            .map(move |line| (line, hl))
                    })
                    .map(|(line, hl)| {
                        let LineCols {
                            line,
                            col_start,
                            col_end,
                        } = line;
                        current_bf.add_highlight(
                            highlight_ns,
                            hl,
                            line,
                            col_start as i64,
                            col_end as i64,
                        )
                    })
                    .collect_vec();

                futures::future::try_join_all(futures).await?;

                // Eval
                let nodes: Vec<_> = arena
                    .enumerate()
                    .filter_map(|(id, node)| {
                        if !node.is(Nodes::Error) {
                            if !node.is(Nodes::Value) {
                                return None;
                            }
                            if node.children.is_empty() {
                                return None;
                            }
                            if node.is_any(&[
                                Nodes::Struct,
                                Nodes::Array,
                                Nodes::String,
                                Nodes::Md_Value,
                            ]) {
                                return None;
                            }
                        }
                        Some(id)
                    })
                    .collect();

                let mut futures = nodes
                    .into_iter()
                    .filter_map(|id| Some((id, db.eval(file_id, id).value.clone()?)))
                    .map(|(id, value)| (arena.get(id), value))
                    .filter_map(|(node, value)| {
                        Some((node.span.lines_cols(&lines).last()?.line, value))
                    })
                    .map(|(line, value)| {
                        api.call_function(
                            "nvim_buf_set_virtual_text",
                            vec![
                                current_bf.into_val(),   // buffer
                                highlight_ns.into_val(), // ns
                                line.into_val(),         // line
                                Value::Array(vec![Value::Array(vec![
                                    Value::String(format!("= {}", &value).into()),
                                    Value::String("Comment".into()),
                                ])]),
                                Value::Map(vec![]),
                            ],
                        )
                    })
                    .collect_vec();

                // Errors
                let current_w = api.get_current_win().await?;

                let diagnostics = dbg!(dbg_buffer, db.all_diagnostics())
                    .into_iter()
                    .filter_map(|(_path, id, error)| {
                        let node = arena.get(id);
                        node.span.lines_cols(&lines).last().map(
                            |LineCols {
                                 line, col_start, ..
                             }| {
                                Diagnostic::new(
                                    &current_bf,
                                    error,
                                    *line,
                                    *col_start,
                                    DiagnosticType::Error,
                                )
                            },
                        )
                    })
                    .collect::<Vec<Diagnostic>>();

                futures.extend(diagnostics.iter().map(|diagnostic| {
                    api.call_function(
                        "nvim_buf_set_virtual_text",
                        vec![
                            current_bf.into_val(),        // buffer
                            highlight_ns.into_val(),      // ns
                            diagnostic.line().into_val(), // line
                            Value::Array(vec![Value::Array(vec![
                                Value::String(diagnostic.text().into()),
                                Value::String("Error".into()),
                            ])]),
                            Value::Map(vec![]),
                        ],
                    )
                }));

                let list = "setloclist";

                futures.push(api.call_function(
                    list,
                    vec![
                        current_w.into_val(),
                        Value::Array(diagnostics.into_iter().map(|d| d.into_val()).collect()),
                        "r".into_val(),
                    ],
                ));
                futures.push(api.call_function(
                    list,
                    vec![
                        current_w.into_val(),
                        Value::Array(vec![]),
                        "a".into_val(),
                        Value::Map(vec![("title".into_val(), "NeuLang Diagnostics".into_val())]),
                    ],
                ));

                futures::future::try_join_all(futures).await?;
                /*


                */

                // api.command("lwindow").await?;

                // Debug window
                //writeln!(&mut dbg_buffer, "```")?;
                //writeln!(&mut dbg_buffer, "{}", buf)?;
                //writeln!(&mut dbg_buffer, "```\n")?;
                //writeln!(&mut dbg_buffer, "{:#?}\n", tokens)?;
                writeln!(&mut dbg_buffer, "{}\n\n", result.display(&buf))?;
                //dbg!(dbg_buffer, arena);

                let debug_lines = dbg_buffer.lines().map(|l| l.to_string()).collect_vec();
                debug_bf.set_lines(0, -1, false, debug_lines).await?;

                Ok(())
            }
            _ => bail!("Wrong Event"),
        }
    }
}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_notify(&self, name: String, args: Vec<Value>, api: Neovim) {
        Self::handle_err(&api, async {
            match name.as_ref() {
                "load" => self.on_load(args, &api).await,
                "nvim_buf_lines_event" => self.on_nvim_buf_lines_event(args, &api).await,
                other => {
                    api.command(&format!(r#"echo "Other: {}""#, other)).await?;
                    Ok(())
                }
            }
        })
        .await;
    }
}
