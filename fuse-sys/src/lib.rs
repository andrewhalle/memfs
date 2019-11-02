use std::convert::TryInto;
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::size_of;
use std::os::raw::c_char;
use std::ptr::null_mut;

mod middle;
mod raw;

pub fn log(s: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/memfs-log.txt")
        .unwrap();
    file.write_all("rust: ".as_bytes()).unwrap();
    file.write_all(s.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
}

pub trait File {
    fn data(&self) -> Vec<u8>;
    fn name(&self) -> String;

    fn size(&self) -> i64 {
        self.data().len().try_into().unwrap()
    }
}

pub trait Directory {
    fn directories(&self) -> Vec<Box<dyn Directory>>;
    fn files(&self) -> Vec<Box<dyn File>>;
    fn name(&self) -> String;
}

pub enum Node {
    File(Box<dyn File>),
    Directory(Box<dyn Directory>),
}

impl Node {
    fn size(&self) -> i64 {
        match self {
            Node::File(f) => f.size(),
            Node::Directory(_) => 0,
        }
    }

    fn is_directory(&self) -> bool {
        match self {
            Node::File(_) => false,
            Node::Directory(_) => true,
        }
    }
}

pub trait FsDataStore {
    fn getdir(&self, path: &str) -> Option<Box<dyn Directory>>;
    fn search(&self, path: &str) -> Option<Node>;
}

pub struct Fs {
    pub data: Box<dyn FsDataStore>,
}

impl Fs {
    pub fn serve(self) {
        let mut hello_oper = middle::get_oper();

        let argc = 2;
        let mut argv = [
            CString::new("memfs").unwrap().into_raw(),
            CString::new("/tmp/memfs").unwrap().into_raw(),
        ];

        unsafe {
            middle::FILES = Some(self);
            raw::fuse_main_real(
                argc,
                &mut argv[0] as *mut *mut c_char,
                &mut hello_oper as *mut raw::fuse_operations,
                size_of::<raw::fuse_operations>(),
                null_mut(),
            );
        }
    }
}
