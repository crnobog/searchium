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
    pub fn new<Globs, S>(root: PathBuf, globs: Globs) -> Self
    where
        Globs: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let mut builder = ignore::gitignore::GitignoreBuilder::new(root);
        for glob in globs.into_iter() {
            if let Err(e) = builder.add_line(None, glob.as_ref()) {
                println!("Error parsing glob line {}: {}", glob.as_ref(), e);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_leading_two_asterisk() {
        let root = PathBuf::from("C:\\projects");
        let filter = PathGlobFilter::new(root, &["**/test"]);

        assert!(filter.matches(PathBuf::from("C:\\projects\\test").as_path(), true));
        assert!(filter.matches(PathBuf::from("C:\\projects\\test\\foo.txt").as_path(), false));
    }
}
