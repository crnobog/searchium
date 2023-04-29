use crate::concurrent_bag::*;
use crate::fs_filter::*;
use std::collections::HashMap;
use std::mem;
use std::path::{Path, PathBuf};
use tracing::{event, Level};

#[allow(dead_code)]
pub struct Root {
    directory: Directory,
    all_files: Vec<PathBuf>,
    searchable_files: Vec<PathBuf>,
}

#[allow(dead_code)]
impl Root {
    pub fn directory(&self) -> &Directory {
        &self.directory
    }
    pub fn all_files(&self) -> &[PathBuf] {
        &self.all_files
    }
    pub fn searchable_files(&self) -> &[PathBuf] {
        &self.searchable_files
    }
}

#[allow(dead_code)]
pub struct Directory {
    dir_path: PathBuf,
    child_directories: Vec<Directory>,
    files: Vec<PathBuf>,
}

impl Directory {
    fn new(dir_path: PathBuf, child_directories: Vec<Directory>, files: Vec<PathBuf>) -> Self {
        Directory {
            dir_path,
            child_directories,
            files,
        }
    }

    pub fn path(&self) -> &Path {
        self.dir_path.as_path()
    }
}

#[derive(PartialEq, Eq, Debug)]
struct DiscoveredDirectory {
    path: PathBuf,
    files: Vec<DiscoveredFile>,
}

#[derive(PartialEq, Eq, Debug)]
struct DiscoveredFile {
    path: PathBuf,
    searchable: bool,
}

pub fn scan_filesystem<P: AsRef<Path> + Send>(
    root_path: P,
    scan_filter: &(impl PathFilter + Sync),
    search_filter: &(impl PathFilter + Sync),
) -> Root {
    let root_path = root_path.as_ref();
    event!(Level::INFO, root = ?root_path, "Scanning filesystem");
    let directories_with_files = Bag::new();
    rayon::scope(|s| {
        s.spawn(|s| {
            scan_directory_recursive(
                root_path,
                scan_filter,
                search_filter,
                (),
                &directories_with_files,
                s,
                true,
            )
        });
    });
    event!(Level::INFO, root = ?root_path, "Building filesystem");
    build_filesystem(root_path, directories_with_files)
}

fn build_filesystem(
    root_path: &Path,
    directories_with_files: impl IntoIterator<Item = DiscoveredDirectory>,
) -> Root {
    let mut directories_with_files: Vec<_> = directories_with_files.into_iter().collect();
    event!(
        Level::INFO,
        "Found {} directories with files",
        directories_with_files.len()
    );
    let directories_to_files = {
        let mut dirs: HashMap<&Path, _> = HashMap::new();
        for dir in &mut directories_with_files {
            let path = dir.path.as_path();
            dirs.insert(path, mem::take(&mut dir.files));
            if *path == *root_path {
                continue;
            }
            for parent in path.ancestors().skip(1) {
                dirs.entry(parent).or_insert(Vec::default());
                if *parent == *root_path {
                    break;
                }
            }
        }
        dirs
    };

    #[derive(Default)]
    struct DirectoryChildren<'a> {
        files: Vec<DiscoveredFile>,
        dirs: Vec<&'a Path>,
    }

    let mut directory_to_children = HashMap::new();
    for (path, mut files) in directories_to_files {
        directory_to_children
            .entry(path)
            .or_insert(DirectoryChildren::default())
            .files = mem::take(&mut files);
        if *path == *root_path {
            continue;
        }
        if let Some(parent) = path.parent() {
            directory_to_children
                .entry(parent)
                .or_insert(DirectoryChildren::default())
                .dirs
                .push(path);
        }
    }

    let mut all_files = Vec::new();
    let mut searchable_files = Vec::new();
    let mut directories: Vec<_> = directory_to_children.into_iter().collect();
    directories.sort_by(|a, b| a.0.cmp(b.0).reverse());
    let mut directory_map: HashMap<&Path, Directory> = HashMap::new();
    for (parent, children) in directories {
        let child_dirs: Vec<_> = children
            .dirs
            .into_iter()
            .map(|child_path| {
                directory_map
                    .remove(child_path)
                    .unwrap_or_else(|| panic!("Missing child {:?} of {:?}", child_path, parent))
            })
            .collect();
        // TODO: avoid copies here? Borrow directory as mut and swap? Refcount path symbols so maps don't have to borrow?
        let files = children.files.iter().map(|f| f.path.clone());
        all_files.extend(files.clone());
        searchable_files.extend(
            children
                .files
                .iter()
                .filter_map(|f| f.searchable.then(|| f.path.clone())),
        );
        directory_map.insert(
            parent,
            Directory::new(parent.to_owned(), child_dirs, files.clone().collect()),
        );
    }
    assert!(directory_map.len() == 1);
    let directory = directory_map.into_values().next().unwrap();
    Root {
        directory,
        all_files,
        searchable_files,
    }
}

enum DirectoryOrFile {
    Directory(PathBuf),
    File(PathBuf),
}

trait DirectoryItem {
    fn path(&self) -> &Path;
    fn is_dir(&self) -> bool;
}

impl DirectoryItem for DirectoryOrFile {
    fn path(&self) -> &Path {
        match self {
            DirectoryOrFile::Directory(p) => p,
            DirectoryOrFile::File(p) => p,
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            DirectoryOrFile::Directory(_) => true,
            DirectoryOrFile::File(_) => false,
        }
    }
}

trait DirScanner {
    type Item: DirectoryItem;
    type Iter: Iterator<Item = Self::Item>;

    fn read_dir(&self, p: &Path) -> Result<Self::Iter, ()>;
}

struct DirIterator(std::fs::ReadDir);

impl Iterator for DirIterator {
    type Item = DirectoryOrFile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.0.next() {
            let item = item.ok()?;
            let info = item.file_type().ok()?;
            Some(if info.is_dir() {
                DirectoryOrFile::Directory(item.path())
            } else {
                DirectoryOrFile::File(item.path())
            })
        } else {
            None
        }
    }
}

impl DirScanner for () {
    type Item = DirectoryOrFile;
    type Iter = DirIterator;

    fn read_dir(&self, p: &Path) -> Result<Self::Iter, ()> {
        match std::fs::read_dir(p) {
            Ok(iter) => Ok(DirIterator(iter)),
            Err(_) => Err(()),
        }
    }
}

fn scan_directory_recursive<'a, FS>(
    dir_path: &Path,
    scan_filter: &'a (impl PathFilter + Sync),
    search_filter: &'a (impl PathFilter + Sync),
    scanner: FS,
    directories_with_files: &'a Bag<DiscoveredDirectory>,
    scope: &rayon::Scope<'a>,
    searchable : bool
) where
    FS: DirScanner + Send + 'a + Clone,
{
    let dir = match scanner.read_dir(dir_path) {
        Ok(dir) => dir,
        Err(_) => return,
    };

    let dir_path = dir_path.to_path_buf();
    let files: Vec<_> = dir
        .into_iter()
        .filter_map(|entry| {
            let path = entry.path().to_owned();
            let is_dir = entry.is_dir();
            if scan_filter.matches(&path, is_dir) {
                // event!(Level::DEBUG, ?path, "Skipping filtered path");
                None
            } else if is_dir {
                let child_searchable = searchable && !search_filter.matches(&path, is_dir);
                // event!(Level::DEBUG, ?path, "Discovered directory");
                let scanner = scanner.clone();
                scope.spawn(move |scope| {
                    scan_directory_recursive(
                        &path,
                        scan_filter,
                        search_filter,
                        scanner,
                        directories_with_files,
                        scope,
                        child_searchable
                    )
                });
                None
            } else {
                // add to this directory
                let searchable = searchable && !search_filter.matches(&path, false);
                // event!(Level::DEBUG, ?path, ?searchable, "Discovered file");
                Some(DiscoveredFile { path, searchable })
            }
        })
        .collect();

    if !files.is_empty() {
        directories_with_files.push(DiscoveredDirectory {
            path: dir_path,
            files,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{collections::HashSet, path::PathBuf};

    fn create_test_data(root: &str, children: Vec<(&str, Vec<&str>)>) -> Vec<DiscoveredDirectory> {
        let root = PathBuf::from(root);
        children
            .into_iter()
            .map(|(dir, files)| {
                let dir = root.join(dir);
                let files: Vec<_> = files
                    .iter()
                    .map(|file| DiscoveredFile {
                        path: dir.join(file),
                        searchable: true,
                    })
                    .collect();
                DiscoveredDirectory {
                    path: dir,
                    files: files,
                }
            })
            .collect()
    }

    #[test]
    fn test_test_data() {
        let root_path = "c:\\code\\project";
        let data = create_test_data(
            root_path,
            vec![
                ("", vec!["readme.md"]),
                ("include", vec!["header.h"]),
                ("src", vec!["main.cpp", "util.cpp"]),
                ("src/module", vec!["module1.cpp", "module2.cpp"]),
            ],
        );

        assert_eq!(data[0].path, PathBuf::from("C:\\code\\project"));
        assert_eq!(
            data[0]
                .files
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<_>>(),
            vec![PathBuf::from("C:\\code\\project\\readme.md")]
        );

        assert_eq!(data[1].path, PathBuf::from("C:\\code\\project\\include"));
        assert_eq!(
            data[1]
                .files
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<_>>(),
            vec![PathBuf::from("C:\\code\\project\\include\\header.h")]
        );

        assert_eq!(data[2].path, PathBuf::from("C:\\code\\project\\src"));
        assert_eq!(
            data[2]
                .files
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<_>>(),
            vec![
                PathBuf::from("C:\\code\\project\\src\\main.cpp"),
                PathBuf::from("C:\\code\\project\\src\\util.cpp")
            ]
        );

        assert_eq!(
            data[3].path,
            PathBuf::from("C:\\code\\project\\src\\module")
        );
        assert_eq!(
            data[3]
                .files
                .iter()
                .map(|f| f.path.clone())
                .collect::<Vec<_>>(),
            vec![
                PathBuf::from("C:\\code\\project\\src\\module\\module1.cpp"),
                PathBuf::from("C:\\code\\project\\src\\module\\module2.cpp")
            ]
        );
    }

    // TODO: Do we need different tests per target platform for different path formats?
    #[test]
    fn test_complete_tree() {
        let root_path = "C:\\code\\projects";
        let discovered_directories = create_test_data(
            root_path,
            vec![
                ("", vec!["readme.md"]),
                ("src", vec!["main.cpp", "module.h"]),
                ("src/module", vec!["module1.cpp", "module2.cpp"]),
            ],
        );
        let root = build_filesystem(PathBuf::from(root_path).as_path(), discovered_directories);
        let dir = root.directory;
        let p = |p| PathBuf::from(p);
        assert_eq!(dir.dir_path, p("C:\\code\\projects"));
        assert_eq!(dir.files, vec![p("C:\\code\\projects\\readme.md")]);
        assert_eq!(dir.child_directories.len(), 1);

        let src = &dir.child_directories[0];
        assert_eq!(src.dir_path, p("C:\\code\\projects\\src"));
        assert_eq!(
            src.files,
            vec![
                p("C:\\code\\projects\\src\\main.cpp"),
                p("C:\\code\\projects\\src\\module.h")
            ]
        );
        assert_eq!(src.child_directories.len(), 1);

        let module = &src.child_directories[0];
        assert_eq!(module.dir_path, p("C:\\code\\projects\\src\\module"));
        assert_eq!(
            module.files,
            vec![
                p("C:\\code\\projects\\src\\module\\module1.cpp"),
                p("C:\\code\\projects\\src\\module\\module2.cpp")
            ]
        );
        assert_eq!(module.child_directories.len(), 0);
    }

    #[test]
    fn test_incomplete_tree() {
        let root_path = "C:\\code\\projects";
        let directories_with_files = create_test_data(
            root_path,
            vec![
                ("", vec!["readme.md"]),
                ("src", vec!["main.cpp", "module.h"]),
                ("src/modules/module1", vec!["module1.cpp"]),
                ("src/modules/module2", vec!["module2.cpp"]),
            ],
        );
        let root = build_filesystem(PathBuf::from(root_path).as_path(), directories_with_files);
        let dir = root.directory;
        let p = |p| PathBuf::from(p);
        assert_eq!(dir.dir_path, p("C:\\code\\projects"));
        assert_eq!(dir.files, vec![p("C:\\code\\projects\\readme.md")]);
        assert_eq!(dir.child_directories.len(), 1);

        let src = &dir.child_directories[0];
        assert_eq!(src.dir_path, p("C:\\code\\projects\\src"));
        assert_eq!(
            src.files,
            vec![
                p("C:\\code\\projects\\src\\main.cpp"),
                p("C:\\code\\projects\\src\\module.h")
            ]
        );
        assert_eq!(src.child_directories.len(), 1);

        let modules = &src.child_directories[0];
        assert_eq!(modules.dir_path, p("C:\\code\\projects\\src\\modules"));
        assert_eq!(modules.files.len(), 0);
        assert_eq!(modules.child_directories.len(), 2);

        let module2 = &modules.child_directories[0];
        assert_eq!(
            module2.dir_path,
            p("C:\\code\\projects\\src\\modules\\module2")
        );
        assert_eq!(
            module2.files,
            vec![p("C:\\code\\projects\\src\\modules\\module2\\module2.cpp")]
        );
        assert_eq!(module2.child_directories.len(), 0);

        let module1 = &modules.child_directories[1];
        assert_eq!(
            module1.dir_path,
            p("C:\\code\\projects\\src\\modules\\module1")
        );
        assert_eq!(
            module1.files,
            vec![p("C:\\code\\projects\\src\\modules\\module1\\module1.cpp")]
        );
        assert_eq!(module1.child_directories.len(), 0);
    }

    enum TestDirOrFile {
        Directory(&'static str),
        File(&'static str),
    }

    impl<'a> DirScanner for &'a HashMap<PathBuf, Vec<TestDirOrFile>> {
        type Item = DirectoryOrFile;
        type Iter = Box<dyn Iterator<Item = Self::Item> + 'a>;

        fn read_dir(&self, p: &Path) -> Result<Self::Iter, ()> {
            match self.get(p) {
                Some(item) => {
                    let parent = p.to_owned();
                    let m = item.iter().map(move |item| {
                        let r = match *item {
                            TestDirOrFile::Directory(s) => {
                                DirectoryOrFile::Directory(parent.join(s))
                            }
                            TestDirOrFile::File(s) => DirectoryOrFile::File(parent.join(s)),
                        };
                        r
                    });
                    Ok(Box::new(m))
                }
                None => Err(()),
            }
        }
    }

    #[test]
    fn test_scan_no_filter() {
        let p = |p| PathBuf::from(p);
        let d = |dir| TestDirOrFile::Directory(dir);
        let f = |f| TestDirOrFile::File(f);

        let filter = ();
        let directories_with_files = Bag::new();
        let root_path = PathBuf::from("C:\\code\\projects");
        let test_data = HashMap::from([
            (p("C:\\code\\projects"), vec![f("readme.md"), d("src")]),
            (
                p("C:\\code\\projects\\src"),
                vec![f("main.cpp"), f("util.h")],
            ),
            // unused as not reported by parent dir
            (
                p("C:\\code\\projects\\docs"),
                vec![f("architecture.md"), f("todo.md")],
            ),
        ]);
        rayon::scope(|s| {
            s.spawn(|s| {
                scan_directory_recursive(
                    &root_path,
                    &filter,
                    &filter,
                    &test_data,
                    &directories_with_files,
                    s,
                    true,
                )
            });
        });

        let mut results: Vec<_> = directories_with_files.into_iter().collect();
        results.sort_by(|a, b| a.path.cmp(&b.path));
        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0],
            DiscoveredDirectory {
                path: p("C:\\code\\projects"),
                files: vec![DiscoveredFile {
                    path: p("C:\\code\\projects\\readme.md"),
                    searchable: true
                }]
            }
        );
        assert_eq!(
            results[1],
            DiscoveredDirectory {
                path: p("C:\\code\\projects\\src"),
                files: vec![
                    DiscoveredFile {
                        path: p("C:\\code\\projects\\src\\main.cpp"),
                        searchable: true
                    },
                    DiscoveredFile {
                        path: p("C:\\code\\projects\\src\\util.h"),
                        searchable: true
                    },
                ]
            }
        );
    }

    impl PathFilter for HashSet<PathBuf> {
        fn matches(&self, p: &Path, _is_dir: bool) -> bool {
            self.contains(p)
        }
    }

    #[test]
    fn test_scan_filter() {
        let p = |p| PathBuf::from(p);
        let d = |dir| TestDirOrFile::Directory(dir);
        let f = |f| TestDirOrFile::File(f);

        let filter = HashSet::from([p("C:\\code\\projects\\generated")]);
        let directories_with_files = Bag::new();
        let root_path = PathBuf::from("C:\\code\\projects");
        let test_data = HashMap::from([
            (
                p("C:\\code\\projects"),
                vec![f("readme.md"), d("src"), d("generated")],
            ),
            (
                p("C:\\code\\projects\\src"),
                vec![f("main.cpp"), f("util.h")],
            ),
            (
                p("C:\\code\\projects\\generated"),
                vec![f("generated1.cpp"), f("generated2.cpp")],
            ),
        ]);
        rayon::scope(|s| {
            s.spawn(|s| {
                scan_directory_recursive(
                    &root_path,
                    &filter,
                    &(),
                    &test_data,
                    &directories_with_files,
                    s,
                    true,
                )
            });
        });

        let mut results: Vec<_> = directories_with_files.into_iter().collect();
        results.sort_by(|a, b| a.path.cmp(&b.path));
        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0],
            DiscoveredDirectory {
                path: p("C:\\code\\projects"),
                files: vec![DiscoveredFile {
                    path: p("C:\\code\\projects\\readme.md"),
                    searchable: true
                }]
            }
        );
        assert_eq!(
            results[1],
            DiscoveredDirectory {
                path: p("C:\\code\\projects\\src"),
                files: vec![
                    DiscoveredFile {
                        path: p("C:\\code\\projects\\src\\main.cpp"),
                        searchable: true
                    },
                    DiscoveredFile {
                        path: p("C:\\code\\projects\\src\\util.h"),
                        searchable: true
                    },
                ]
            }
        );
    }
}
