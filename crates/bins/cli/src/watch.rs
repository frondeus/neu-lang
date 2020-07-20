use crate::build::Builder;
use anyhow::Result;
use ignore::gitignore::GitignoreBuilder;
use neu_syntax::db::{FileId, FileKind};
use notify::DebouncedEvent;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub fn watch(
    db: &mut dyn Builder,
    root: &Path,
    dist: &Path,
    mut hot_tx: Option<UnboundedSender<()>>,
) -> Result<()> {
    use notify::{RecommendedWatcher, RecursiveMode, Watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;

    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

    watcher.watch(root, RecursiveMode::Recursive)?;

    let mut ignore_builder = GitignoreBuilder::new(root);
    ignore_builder.add(root.join(".gitignore"));
    ignore_builder.add_line(None, ".gitignore")?;
    let gitignore = ignore_builder.build()?;

    crate::build::scan_all(db, &root)?;
    db.build_all(root.into(), dist.into())?;

    println!("Build finished. Watching directory");
    let mut hotreload = || {
        if let Some(hot_tx) = hot_tx.as_mut() {
            hot_tx.send(()).expect("Hot reload send");
        }
    };

    loop {
        let event = rx.recv()?;
        match event {
            DebouncedEvent::NoticeWrite(path)
            | DebouncedEvent::Create(path)
            | DebouncedEvent::Write(path) => {
                let is_dir = path.is_dir();
                let matches = gitignore.matched_path_or_any_parents(&path, is_dir);
                if !matches.is_ignore() && path.exists() {
                    println!("\n\n\nChanged: {:?}", path);
                    let path_str = path.display().to_string();
                    let file_id: FileId = db.file_id((path_str, FileKind::Md));
                    //TODO: Only mds for now
                    let mut all_mds = (*db.all_mds()).clone();
                    if !all_mds.contains(&file_id) {
                        println!("File did not existed");
                        all_mds.insert(file_id);
                        db.set_all_mds(Arc::new(all_mds));
                    }
                    let file = std::fs::read_to_string(path)?;
                    db.set_input(file_id, Arc::new(file));
                    db.build_all(root.into(), dist.into())?;
                    hotreload();
                }
            }
            DebouncedEvent::NoticeRemove(path) | DebouncedEvent::Remove(path) => {
                let is_dir = path.is_dir();
                let matches = gitignore.matched_path_or_any_parents(&path, is_dir);
                if !matches.is_ignore() && !path.exists() {
                    let file_id = db.file_id((path.display().to_string(), FileKind::Md));
                    let mut all_mds = (*db.all_mds()).clone();
                    if all_mds.contains(&file_id) {
                        println!("\n\n\nRemoved: {:?}", path);
                        all_mds.remove(&file_id);
                        db.set_all_mds(Arc::new(all_mds));
                        db.build_all(root.into(), dist.into())?;
                        hotreload();
                    }
                }
            }
            DebouncedEvent::Rename(_, _) => {
                //todo!("Rename!");
                log::warn!("Not implemented yet: Rename");
                crate::build::scan_all(db, &root)?;
                db.build_all(root.into(), dist.into())?;
                hotreload();
            }
            DebouncedEvent::Chmod(_) | DebouncedEvent::Error(_, _) => (),
            DebouncedEvent::Rescan => {
                println!("Have to rescan");
                crate::build::scan_all(db, &root)?;
                db.build_all(root.into(), dist.into())?;
                hotreload();
            }
        }
    }
}
