use std::collections::HashMap;

#[derive(Default)]
pub struct RWEPerm {
    read: bool,
    write: bool,
    exec: bool,
}

#[derive(Default)]
pub struct Permissions {
    owner: RWEPerm,
    group: RWEPerm,
    everyone: RWEPerm,
}

#[derive(Default)]
pub struct Ownership {
    owner: usize,
    group: usize
}

pub struct File {
    data: Vec<u8>,
    owner: Ownership,
    permissions: Permissions,
}

pub struct Directory {
    files: HashMap<String, File>,
    directories: HashMap<String, Directory>,
}

pub struct Filesystem {
    root: Directory
}

impl File {
    pub fn new(init_data: &[u8]) -> File {
        let mut data = Vec::with_capacity(init_data.len());
        for &b in init_data.iter() {
            data.push(b);
        }

        File {
            data,
            owner: Ownership::default(),
            permissions: Permissions::default(),
        }
    }

    pub fn empty() -> File {
        File {
            data: Vec::new(),
            owner: Ownership::default(),
            permissions: Permissions::default(),
        }
    }
}

impl Directory {
    pub fn new() -> Directory {
        Directory { files: HashMap::new(), directories: HashMap::new() }
    }

    pub fn touch(&mut self, filename: &str) {
        self.files.insert(filename.to_string(), File::empty());
    }
}

impl Filesystem {
    pub fn new() -> Filesystem {
        Filesystem { root: Directory::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::File;

    #[test]
    fn it_works() {
        let _f = File::new("hello, world!\n".as_bytes());
    }
}
