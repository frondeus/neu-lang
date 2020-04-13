use itertools::Itertools;
use async_trait::async_trait;
use rmpv::Value;
use tokio::io::Stdout;
use nvim_rs::{compat::tokio::Compat, Handler};
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::state::State;
use crate::highlight::NodeHighlight;
use anyhow::{Result, bail};
use std::future::Future;
use crate::Neovim;
use crate::diagnostic::{Diagnostic, DiagnosticType};
use nvim_rs::rpc::IntoVal;
use crate::span_ext::{TextRangeExt, LineCols};

#[derive(Clone, Default)]
pub struct NeovimHandler {
    state: Arc<RwLock<Option<State>>>
}

impl NeovimHandler {
    async fn handle_err(api: &Neovim, e: impl Future<Output=Result<()>>) {
        match e.await {
            Ok(_) => (),
            Err(e) => {
                log::error!("{:?}", &e);
                api.err_writeln(&format!("{:?}", e)).await.expect("Couldn't send error");
            }
        }
    }

    async fn on_load(&self, _args: Vec<Value>, api: &Neovim) -> Result<()> {
        let buf = api.get_current_buf().await?;


        if self.state.read().await.is_none() {
            api.command("vsp").await?;
            api.command("wincmd l").await?;

            let debug_bf = api.create_buf(true, true).await?;
            debug_bf.set_name("**NeuLang Debug**").await?;
            let win = api.get_current_win().await?;
            win.set_buf(&debug_bf).await?;

            let highlight_ns = api.create_namespace("NeuLang Highlight").await?;

            *self.state.write().await = Some(State::new(debug_bf, highlight_ns));

            api.command("wincmd h").await?;
            //api.command("cope").await?;
            api.command(r#"echo "NeuLang Loaded""#).await?;
        }

        buf.attach(true, vec![]).await?;

        Ok(())
    }

    async fn on_nvim_buf_lines_event(&self, args: Vec<Value>, api: &Neovim) -> Result<()> {
        match &args[..] {
            [_cbf, _tick, _first_line, _last_line, Value::Array(_changed), Value::Boolean(_more)] => {
                use std::fmt::Write;
                let mut dbg_buffer = String::new();

                let State {
                    debug_bf,
                    highlight_ns
                } = self.state.read().await.clone().expect("State");

                let current_bf = api.get_current_buf().await?;
                let lines = current_bf.get_lines(0, -1, false).await?;
                let buf = lines.iter().join("\n");

                let parse_result = {
                    use neu_parser::core::{Lexer, State};
                    use neu_parser::{parser};

                    //let tokens: Vec<_> = Lexer::new(&buf).map(|t| t.display(&buf, true).to_string()).collect();
                    State::parse(Lexer::new(&buf), parser())
                };

                // Highlighting
                current_bf.clear_namespace(highlight_ns, 0, -1).await?;

                for node in parse_result.nodes.iter() {
                    if let Some(hl_group) = node.highlight() {
                        for LineCols { line, col_start, col_end } in node.span.lines_cols(&lines) {
                            current_bf.add_highlight(highlight_ns, hl_group, line, col_start as i64, col_end as i64).await?;
                        }
                    }
                }

                // Errors
                let current_w = api.get_current_win().await?;

                let diagnostics = parse_result.errors.iter().filter_map(|(id, error)| {
                    let node = parse_result.nodes.get(id);
                    if let Some(LineCols { line, col_start, .. }) = node.span.lines_cols(&lines).first() {
                        Some(Diagnostic::new(&current_bf, error.display(&buf).to_string(), *line + 1, *col_start + 1, DiagnosticType::Error))
                    } else {
                        None
                    }
                }).collect::<Vec<Diagnostic>>();

                let list = "setloclist";

                api.call_function(list, vec![
                    current_w.into_val(),
                    Value::Array(diagnostics.into_iter().map(|d| d.into_val()).collect()),
                    "r".into_val()
                ]).await?;
                api.call_function(list, vec![
                    current_w.into_val(),
                    Value::Array(vec![]),
                    "a".into_val(),
                    Value::Map(vec![
                        ("title".into_val(), "NeuLang Diagnostics".into_val())
                    ])
                ]).await?;

                api.command("lwindow").await?;

                // Debug window
                //writeln!(&mut dbg_buffer, "```")?;
                //writeln!(&mut dbg_buffer, "{}", buf)?;
                //writeln!(&mut dbg_buffer, "```\n")?;
                //writeln!(&mut dbg_buffer, "{:#?}\n", tokens)?;
                writeln!(&mut dbg_buffer, "{}\n", parse_result.display(&buf))?;
                writeln!(&mut dbg_buffer, "\n{:#?}", parse_result.nodes)?;

                let debug_lines = dbg_buffer.lines().map(|l| l.to_string()).collect_vec();
                debug_bf.set_lines(0, -1, false, debug_lines).await?;


                Ok(())
            },
            _ => bail!("Wrong Event")
        }
    }
}

#[async_trait]
impl Handler for NeovimHandler {
    type Writer = Compat<Stdout>;

    async fn handle_notify(
        &self,
        name: String,
        args: Vec<Value>,
        api: Neovim,
    ) {
        Self::handle_err(&api, async {
            match name.as_ref() {
                "load" => self.on_load(args, &api).await,
                "nvim_buf_lines_event" => self.on_nvim_buf_lines_event(args, &api).await,
                other => {
                    api.command(&format!(r#"echo "Other: {}""#, other)).await?;
                    Ok(())
                }
            }
        }).await;
    }
}

