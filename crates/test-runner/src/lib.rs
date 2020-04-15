mod format_changeset;

use anyhow::{Result, anyhow, bail};
use std::path::{PathBuf, Path};
use regex::Regex;
use difference::{Changeset, Difference};
use crate::format_changeset::format_changeset;
use std::fmt;
use std::io::Write;
use std::time::SystemTime;
use chrono::{DateTime, Local};

struct Comparison(Changeset);
impl fmt::Display for Comparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_changeset(f, &self.0)
    }
}

impl Comparison {
    fn changes(&self) -> &Changeset {
        &self.0
    }
}

fn assert_section(entry: Entry, actual: String) -> Result<()> {
    let mut new_snap_path: PathBuf = entry.entry.into();
    let ext = format!("{}.new", entry.section_name);
    new_snap_path.set_extension(&ext);

    let expected = entry.section;

    if expected != actual {
        let changeset = Comparison(Changeset::new(expected, &actual, "\n"));
        eprintln!("\n```\n{}\n```", entry.input);
        eprintln!("\n{}", changeset);

        std::fs::File::create(&new_snap_path)
            .and_then(|mut file| {
                let expected = expected.lines().collect::<Vec<_>>();
                let actual = actual.lines().collect::<Vec<_>>();
                let datetime: DateTime<Local> = entry.modified.into();
                let dt = datetime.format("%F %T %z");

                let entry_basename = entry.entry.file_name().unwrap().to_string_lossy();
                let snap_basename = new_snap_path.file_name().unwrap().to_string_lossy();

                writeln!(file, "{}", entry.input)?;
                writeln!(file, "--- {} {}", entry_basename, dt)?;
                writeln!(file, "+++ {} {}", snap_basename, dt)?;
                writeln!(file, "@@ -{},{} +{},{} @@", entry.line, expected.len()+1, entry.line, actual.len()+1)?;
                writeln!(file, " [{}]", entry.section_name)?;
                for diffs in changeset.changes().diffs.iter() {
                    let (text, mark) = match diffs {
                        Difference::Same(text)  => (text, " "),
                        Difference::Rem(text)  => (text, "-"),
                        Difference::Add(text)  => (text, "+"),
                    };
                    for line in text.lines() {
                        writeln!(file, "{}{}", mark, line)?;
                    }
                }
                Ok(())
            })
            ?;

        bail!("Not matching");
    }
    else if new_snap_path.exists() {
        std::fs::remove_file(new_snap_path)?;
    }

    Ok(())
}

struct Entry<'a> {
    section_name: &'a str,
    line: usize,
    section: &'a str,
    input: &'a str,
    entry: &'a Path,
    modified: SystemTime
}

pub fn test_snapshots(section_name: &str, f: impl Fn(&str) -> String) -> Result<()> {
    let section_regex = Regex::new(r"^\s*\[([[:alpha:]]+)\]\s*$")?;
    let path = go_to_root()?;
    let mut successes = 0;
    let mut processed = 0;
    let mut skipped = 0;
    for entry in glob::glob(&format!("{}/tests/**/*.neu.snap",path.display()))? {
        let entry = entry?;
        let entry_file = load_file(&entry)?;
        let (input, snaps) = get_source(&entry_file)?;
        let mut section = None;
        let mut from = 0;
        let mut to = snaps.len();
        let input_len = input.lines().count() + 2;
        for (line_idx, line) in snaps.lines().enumerate() {
            if let Some(captures) = section_regex.captures(line) {
                let offset = offset(snaps, line);
                if section.is_some() {
                    to = offset;
                    break;
                }
                let name = captures.get(1).unwrap().as_str();
                if name == section_name {
                    from = offset + line.len();
                    if from < snaps.len() {
                        if &snaps[from..from+1] == "\r" { from += 2; } else { from += 1 }
                    }
                    section = Some(input_len + line_idx + 1);
                }
            }
        }
        if let Some(line) = section {
            let metadata = std::fs::metadata(&entry)?;
            let e = Entry {
                entry: &entry,
                input,
                section_name,
                section: &snaps[from..to],
                line,
                modified: metadata.modified()?
            };
            let actual = format!("{}\n\n", f(input));
            match assert_section(e, actual) {
                Ok(_) => {
                    successes += 1;
                    eprint!(".");
                },
                Err(e) => {
                    eprintln!("{}: {:?}", entry.display(), e);
                }
            }
            processed += 1;
        }
        else {
            skipped += 1;
        }
    }
    eprintln!("\nProcessed {}: {}, Failed: {}, Skipped: {}", section_name, processed, processed - successes, skipped);
    if successes != processed  {
        bail!("Some tests failed");
    }
    Ok(())
}

fn offset(parent: &str, child: &str) -> usize {
    let parent_ptr = parent.as_ptr() as usize;
    let child_ptr = child.as_ptr() as usize;
    child_ptr - parent_ptr
}

fn get_source(file: &String) -> Result<(&str, &str)> {
    let splited = file.split("```\n").collect::<Vec<_>>();
    if splited.len() != 3 { bail!("Expected one source wrapped in ```") }
    let input = splited[1].trim_end_matches('\n');//.trim();
    let sections = splited[2];
    Ok((input, sections))
}

fn load_file(entry: &Path) -> Result<String> {
    let s = std::fs::read_to_string(entry)?;
    Ok(s)
}

fn go_to_root() -> Result<PathBuf> {
    let mut path = std::env::current_dir()?;

    while !path.join("Cargo.lock").exists() {
        path = path.parent().ok_or_else(|| anyhow!("Couldn't find parent directory"))?.into();
    }

    path = path.canonicalize()?;

    Ok(path)
}