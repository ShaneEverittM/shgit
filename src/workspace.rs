use std::collections::HashSet;
use std::io::Result as IOResult;
use std::path::{Path, PathBuf};

pub struct Workspace {
    path: PathBuf,
    ignore: HashSet<PathBuf>,
}

impl Workspace {
    pub fn in_dir<P: Into<PathBuf>>(path_like: P) -> Self {
        let path: PathBuf = path_like.into();
        let ignore = {
            let mut i = HashSet::new();
            i.insert(path.join("."));
            i.insert(path.join(".."));
            i.insert(path.join(".shgit"));
            i
        };

        Self { path, ignore }
    }

    pub fn list_files(&self) -> IOResult<HashSet<PathBuf>> {
        let entries = std::fs::read_dir(&self.path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<HashSet<_>, std::io::Error>>()?;
        Ok(entries
            .difference(&self.ignore)
            .map(PathBuf::from)
            .collect())
    }

    pub fn read_file<P: AsRef<Path>>(&self, path: P) -> IOResult<Vec<u8>> {
        std::fs::read(path)
    }
}
