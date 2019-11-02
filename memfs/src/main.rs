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
        if path == "/" {
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
        Vec::new()
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Clone)]
pub struct MemFsDirectory {
    name: String,
    files: Vec<MemFsFile>,
}

impl MemFsDirectory {
    pub fn new(name: &str) -> MemFsDirectory {
        MemFsDirectory {
            name: String::from(name),
            files: Vec::new(),
        }
    }

    pub fn search(&self, path: &str) -> Option<Node> {
        // XXX actually implement
        if path == "/" {
            Some(Node::Directory(Box::new(self.clone())))
        } else if path == "/hello.txt" {
            Some(Node::File(Box::new(self.files[0].clone())))
        } else {
            None
        }
    }

    pub fn add_file(&mut self, f: MemFsFile) {
        self.files.push(f);
    }
}

impl Directory for MemFsDirectory {
    fn directories(&self) -> Vec<Box<dyn Directory>> {
        Vec::new()
    }

    fn files(&self) -> Vec<Box<dyn File>> {
        let mut retval: Vec<Box<dyn File>> = Vec::new();
        for file in self.files.iter() {
            retval.push(Box::new(file.clone()));
        }

        retval
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let fs_data = MemFs::new().add_file(MemFsFile {
        name: String::from("hello.txt"),
        contents: String::from("hello world!!"),
    });

    let fs = Fs {
        data: Box::new(fs_data),
    };

    // start fs process
    fs.serve();
}
