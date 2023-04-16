use std::path::{Path, PathBuf};

pub trait PathFilter {
    fn matches(&self, p: &Path, is_dir: bool) -> bool;
}

impl PathFilter for () {
    fn matches(&self, _p: &Path, _is_dir: bool) -> bool {
        false
    }
}

pub struct PathGlobFilter {
    ignore: ignore::gitignore::Gitignore,
}

impl PathGlobFilter {
    pub fn new(root: PathBuf, globs: Vec<String>) -> Self {
        let mut builder = ignore::gitignore::GitignoreBuilder::new(root);
        for glob in &globs {
            match builder.add_line(None, glob) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
        PathGlobFilter {
            ignore: builder
                .build()
                .unwrap_or(ignore::gitignore::Gitignore::empty()),
        }
    }
}

impl PathFilter for PathGlobFilter {
    fn matches(&self, p: &Path, is_dir: bool) -> bool {
        !self.ignore.matched(p, is_dir).is_none()
    }
}

// TODO: tests 