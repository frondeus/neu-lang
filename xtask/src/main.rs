use std::path::PathBuf;
use xshell::{cmd, read_file, pushd, rm_rf};

use anyhow::{Result, anyhow, Context};
use microtree_codegen::codegen;
use pico_args::Arguments;

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let subcmd = args.subcommand()?.unwrap_or_default();

    goto_root()?;

    match subcmd.as_str() {
        "codegen" => {
            codegen(
                "crates/lang/syntax/src/ast/neu.config.json",
                "crates/lang/syntax/src/ast/neu.ungram",
                "crates/lang/syntax/src/ast/generated/",
            )?;
        },
        "review" => {
            review()?;
        },
        _ => eprintln!("cargo xtask codegen"),
    }

    Ok(())
}

fn review() -> Result<()> {
    use crossterm::event::{self, Event, KeyCode, KeyEvent};
    use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

    for file in glob::glob("./tests/**/*.new")? {
        let filename = file?;
        let dir = filename.parent().context("Could not find directory for .new")?;
        let file = read_file(&filename)?;
        println!("Accepting: {:?}", &filename);
        println!("-----");
        let diff = cmd!("colordiff")
            .stdin(file.as_bytes())
            .read()?;

        println!("{}\n\n", diff);
        println!("-----");
        loop {
            println!("[Aa]ccept, [Rr]eject, [Ss]kip or [Qq]uit");

            enable_raw_mode()?;
            let event = event::read()?;
            disable_raw_mode()?;
            if let Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            }) =  event {
                match c {
                    'a' | 'A' => {
                        let _p = pushd(&dir);

                        cmd!("patch --ignore-whitespace")
                            .stdin(file.as_bytes())
                            .read()?;

                        break;
                    },
                    'r' | 'R' => {
                        println!("Rejecting change");
                        rm_rf(&filename)?;
                        break;
                    },
                    's' | 'S' => {
                        println!("Skipping");
                        break;
                    },
                    'q' | 'Q' => {
                        println!("Quitting");
                        return Ok(());
                    }
                    _ => continue
                }
            }
        }
    }
    println!("All processed");
    Ok(())
}

fn goto_root() -> Result<()> {
    let git = PathBuf::from(".git");
    loop {
        if git.exists() {
            break Ok(());
        }
        let cwd = std::env::current_dir()?;
        let parent = cwd.parent().ok_or_else(|| anyhow!("Could not find .git root"))?;
        std::env::set_current_dir(parent)?;
    }
}
