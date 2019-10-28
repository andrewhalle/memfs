use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::c_char;
use std::ptr::null_mut;

mod middle;
mod raw;

pub struct Fs {
    msg: String,
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
            middle::MSG = Some(self.msg.clone());
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

pub struct FsBuilder {}

impl FsBuilder {
    pub fn new() -> FsBuilder {
        FsBuilder {}
    }

    pub fn finish(self) -> Fs {
        Fs {
            msg: String::from("goodbye, world\n"),
        }
    }
}
