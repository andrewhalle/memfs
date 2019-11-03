use std::convert::TryInto;
use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_char;
use std::ptr::null_mut;

mod middle;
mod raw;

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

        let mut argv = [CString::new("memfs").unwrap().into_raw()];
        let mut args = raw::fuse_args {
            argc: 1,
            argv: &mut argv[0] as *mut *mut c_char,
            allocated: 0,
        };
        unsafe {
            middle::FILES = Some(self);
            let handle = raw::fuse_new(
                &mut args as *mut raw::fuse_args,
                &mut hello_oper as *mut raw::fuse_operations,
                size_of::<raw::fuse_operations>(),
                null_mut(),
            );
            raw::fuse_mount(handle, CString::new("/tmp/memfs").unwrap().into_raw());
            raw::fuse_loop(handle);
        }
    }
}
