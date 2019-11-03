use std::collections::HashMap;

use fuse::{Directory, File, Fs, FsDataStore, Node};

pub struct MemFs {
    root: MemFsDirectory,
}

impl MemFs {
    fn new() -> MemFs {
        MemFs {
            root: MemFsDirectory::new(""),
        }
    }

    fn add_file(mut self, f: MemFsFile) -> MemFs {
        self.root.add_file(f);
        self
    }
}

impl FsDataStore for MemFs {
    fn getdir(&self, path: &str) -> Option<Box<dyn Directory>> {
        if path != "/" {
            return None;
        }

        Some(Box::new(self.root.clone()))
    }

    fn search(&self, path: &str) -> Option<Node> {
        println!("successfully calling into search, path is {}", path);
        if path == "/" {
            println!("returning root directory");
            return Some(Node::Directory(Box::new(self.root.clone())));
        } else {
            return self.root.search(path);
        }
    }
}

#[derive(Clone)]
pub struct MemFsFile {
    name: String,
    contents: String,
}

impl File for MemFsFile {
    fn data(&self) -> Vec<u8> {
        self.contents.clone().into_bytes()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone)]
pub struct MemFsDirectory {
    name: String,
    files: HashMap<String, MemFsFile>,
    directories: HashMap<String, MemFsDirectory>,
}

// Path helper functions for search.
fn get_leading_entry(path: &str) -> &str {
    let mut retval = path;
    if let Some(0) = retval.find("/") {
        retval = &retval[1..];
    }

    if let Some(index) = retval.find("/") {
        &retval[0..index]
    } else {
        retval
    }
}

fn get_remaining(path: &str) -> &str {
    let mut retval = path;
    if let Some(0) = retval.find("/") {
        retval = &retval[1..];
    }

    if let Some(index) = retval.find("/") {
        &retval[index..]
    } else {
        ""
    }
}

impl MemFsDirectory {
    pub fn new(name: &str) -> MemFsDirectory {
        MemFsDirectory {
            name: String::from(name),
            files: HashMap::new(),
            directories: HashMap::new(),
        }
    }

    pub fn search(&self, path: &str) -> Option<Node> {
        if path == "/" {
            Some(Node::Directory(Box::new(self.clone())))
        } else {
            if self.files.contains_key(get_leading_entry(path)) {
                Some(Node::File(Box::new(
                    self.files.get(get_leading_entry(path)).unwrap().clone(),
                )))
            } else if self.directories.contains_key(get_leading_entry(path)) {
                self.directories
                    .get(get_leading_entry(path))
                    .unwrap()
                    .search(get_remaining(path))
            } else {
                None
            }
        }
    }

    pub fn add_file(&mut self, f: MemFsFile) {
        self.files.insert(f.name.clone(), f);
    }
}

impl Directory for MemFsDirectory {
    fn directories(&self) -> Vec<Box<dyn Directory>> {
        let mut retval: Vec<Box<dyn Directory>> = Vec::new();
        for (_, dir) in self.directories.iter() {
            retval.push(Box::new(dir.clone()));
        }

        retval
    }

    fn files(&self) -> Vec<Box<dyn File>> {
        let mut retval: Vec<Box<dyn File>> = Vec::new();
        for (_, file) in self.files.iter() {
            retval.push(Box::new(file.clone()));
        }

        retval
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let fs_data = MemFs::new()
        .add_file(MemFsFile {
            name: String::from("hello.txt"),
            contents: String::from("hello world!!"),
        })
        .add_file(MemFsFile {
            name: String::from("goodbye.txt"),
            contents: String::from("goodbye world!!"),
        });

    let fs = Fs {
        data: Box::new(fs_data),
    };

    // start fs process
    fs.serve();
}
