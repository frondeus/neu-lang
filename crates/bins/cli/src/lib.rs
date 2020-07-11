use anyhow::anyhow;
use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn find_in_ancestors(start: Option<PathBuf>, indicator: impl AsRef<Path>) -> Result<PathBuf> {
    let mut path = match start {
        Some(s) => s,
        None => std::env::current_dir()?,
    };

    while !path.join(indicator.as_ref()).exists() {
        path = path
            .parent()
            .ok_or_else(|| anyhow!("Couldn't find {} directory", indicator.as_ref().display()))?
            .into();
    }

    path = path.canonicalize()?;

    log::debug!("Found directory at: {}", path.display());
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod find_root_directory_tests {
        use super::*;
        use assert_fs::prelude::*;

        #[test]
        fn neu_exists_in_root_dir() -> Result<()> {
            let temp = assert_fs::TempDir::new()?;
            let neu_dir = temp.child(".neu");
            neu_dir.create_dir_all()?;

            let temp_path = temp.path().canonicalize()?;

            let cwd = find_in_ancestors(Some(temp.path().into()), ".neu")?;

            assert_eq!(cwd, temp_path);

            temp.close()?;
            Ok(())
        }

        #[test]
        fn neu_exists_in_child_dir() -> Result<()> {
            let temp = assert_fs::TempDir::new()?;
            let neu_dir = temp.child(".neu");
            neu_dir.create_dir_all()?;
            let child_dir = temp.child("foo");
            child_dir.create_dir_all()?;

            let temp_path = temp.path().canonicalize()?;

            let cwd = find_in_ancestors(Some(child_dir.path().into()), ".neu")?;

            assert_eq!(cwd, temp_path);

            temp.close()?;
            Ok(())
        }
    }
}
