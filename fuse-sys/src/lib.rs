use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_char;
use std::ptr::null_mut;

mod middle;
mod raw;

pub trait File {
    pub fn data(&self) -> Vec<u8>;

    pub fn size(&self) -> usize {
        self.data().len()
    }
}

pub trait Directory {
    pub fn directories(&self) -> Vec<Box<dyn Directory>>;
    pub fn files(&self) -> Vec<Box<dyn File>>;
    pub fn size(&self) -> usize {
        0
    }
}

pub enum Node {
    File(Box<dyn File>),
    Directory(Box<dyn Directory>),
}

pub trait FsDataStore {
    pub fn getdir(&self, path: &str) -> Box<dyn Directory>;
    pub fn search(&self, path: &str) -> Option<Node>;
}

pub struct Fs {
    data: Box<dyn FsDataStore>,
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
