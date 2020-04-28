use anyhow::{Result, anyhow, bail};
use std::path::{PathBuf, Path};
use regex::Regex;
use std::io::Write;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use diff_utils::{Comparison, PatchOptions, DisplayOptions};

fn assert_section(entry: Entry, actual: String) -> Result<()> {
    let mut new_snap_path: PathBuf = entry.entry.into();
    let ext = format!("{}.new", entry.section_name);
    new_snap_path.set_extension(&ext);

    let expected = entry.section;

    if expected != actual {
        let expected_lines = expected.lines().collect::<Vec<_>>();
        let actual_lines = actual.lines().collect::<Vec<_>>();
        let comparison = Comparison::new(&expected_lines, &actual_lines).compare()?;
        eprintln!("\nFound mismatch in section [{}] in {}\n{}", entry.section_name, entry.entry.display(),
                  comparison.display(DisplayOptions {
                      offset: entry.line,
                      .. Default::default()
                  }));

        std::fs::File::create(&new_snap_path)
            .and_then(|mut file| {
                let datetime: DateTime<Local> = entry.modified.into();
                let dt = datetime.format("%F %T %z");

                let entry_basename = entry.entry.file_name().unwrap().to_string_lossy();
                let snap_basename = new_snap_path.file_name().unwrap().to_string_lossy();

                writeln!(file, "```")?;
                writeln!(file, "{}", entry.input)?;
                writeln!(file, "```")?;
                write!(file, "{}", comparison.patch(
                    entry_basename, &dt,
                    snap_basename, &dt,
                    PatchOptions {
                        offset: entry.line,
                    }
                ))
            })
            ?;

        bail!("failed");
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

pub fn test_snapshots(ext: &str, section_name: &str, f: impl Fn(&str) -> String) -> Result<()> {
    let section_regex = Regex::new(r"^\s*\[([[:alpha:]\.-_]+)\]\s*$")?;
    let path = go_to_root()?;
    let mut successes = 0;
    let mut processed = 0;
    let mut skipped = 0;
    for entry in glob::glob(&format!("{}/tests/**/*.{}.snap", path.display(), ext))? {
        let entry = entry?;
        let entry_file = load_file(&entry)?;
        let (input, snaps) = get_source(&entry_file)?;
        let mut section = None;
        let mut from = 0;
        let mut to = snaps.len();
        let mut last_line = None;
        let input_len = input.lines().count() + 2;
        for (line_idx, line) in snaps.lines().enumerate() {
            if let Some(captures) = section_regex.captures(line) {
                let offset = offset(snaps, line);
                if section.is_some() {
                    to = offset + line.len();
                    last_line = Some(offset);
                    break;
                }
                let name = captures.get(1).unwrap().as_str();
                if name == section_name {
                    from = offset;
                    section = Some(input_len + line_idx);
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
            let last_line = match last_line {
                Some(from) => &snaps[from..to],
                None => &snaps[to..to]
            };
            let actual = format!("[{}]\n{}\n\n{}", section_name, f(input), last_line);
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

fn get_source(file: &str) -> Result<(&str, &str)> {
    let iter = file.chars();
    let bt_count = iter.take_while(|c| *c == '`').count();
    let pat = format!("{:`>width$}", "\n", width = bt_count + 1);
    let splited = file.split(&pat).collect::<Vec<_>>();
    if splited.len() != 3 { bail!("Expected one source wrapped in ```: {}", file) }
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