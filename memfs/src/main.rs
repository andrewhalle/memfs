use fuse::{Directory, File, Fs, FsDataStore, Node};

pub struct MemFs {
    root: MemFsDirectory,
}

impl Fs for MemFs {
    pub fn getdir(&self, path: &str) -> Option<Box<dyn Directory>> {
        if path != "/" {
            return None;
        }

        Some(Box::new(root.clone()))
    }

    pub fn search(&self, path: &str) -> Option<Node> {
        if path == "/" {
            return Some(Node::Directory(Box::new(self.root.clone())));
        } else {
            return self.root.search(path);
        }
    }
}

pub struct MemFsFile {
    name: String,
    contents: String,
}

pub struct MemFsDirectory {}

impl MemFsDirectory {
    pub fn search(&self, path: &str) -> Option<Node> {}
}

fn main() {
    let fs_data = MemFs::new().add_file(MemFsFile {
        name: String::new("hello.txt"),
        contents: String::new("hello world!!"),
    });

    let fs = Fs {
        data: Box::new(fs_data),
    };

    // start fs process
    fs.serve();
}
