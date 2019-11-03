use libc::{memcpy, memset, off_t, size_t, S_IFDIR, S_IFREG};
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null_mut;

use super::{raw, Node};
use crate::Fs;

pub static mut FILES: Option<Fs> = None;

pub unsafe extern "C" fn fuse_init(
    _conn: *mut raw::fuse_conn_info,
    cfg: *mut raw::fuse_config,
) -> *mut c_void {
    println!("hello_init called");
    (*cfg).kernel_cache = 1;
    null_mut()
}

pub unsafe extern "C" fn fuse_readdir(
    _path: *const c_char,
    buf: *mut c_void,
    filler: raw::fuse_fill_dir_t,
    _offset: raw::off_t,
    _fi: *mut raw::fuse_file_info,
    _flags: raw::fuse_readdir_flags,
) -> c_int {
    println!("hello_readdir called");
    filler.unwrap()(buf, CString::new(".").unwrap().as_ptr(), null_mut(), 0, 0);
    filler.unwrap()(buf, CString::new("..").unwrap().as_ptr(), null_mut(), 0, 0);
    let dir = FILES
        .as_ref()
        .unwrap()
        .data
        .getdir(CStr::from_ptr(_path).to_str().unwrap())
        .unwrap();
    for directory in dir.directories() {
        filler.unwrap()(
            buf,
            CString::new(directory.name().as_bytes()).unwrap().as_ptr(),
            null_mut(),
            0,
            0,
        );
    }
    for file in dir.files() {
        filler.unwrap()(
            buf,
            CString::new(file.name().as_bytes()).unwrap().as_ptr(),
            null_mut(),
            0,
            0,
        );
    }

    0
}

pub unsafe extern "C" fn fuse_getattr(
    path: *const c_char,
    stbuf: *mut raw::stat,
    _fi: *mut raw::fuse_file_info,
) -> c_int {
    println!("fuse_getattr called");
    memset(stbuf as *mut c_void, 0, size_of::<raw::stat>());
    let node = FILES
        .as_ref()
        .unwrap()
        .data
        .search(CStr::from_ptr(path).to_str().unwrap())
        .unwrap();
    if node.is_directory() {
        println!("inside is_directory");
        (*stbuf).st_mode = 0o755 | S_IFDIR;
        (*stbuf).st_nlink = 2;
    } else {
        (*stbuf).st_mode = 0o755 | S_IFREG;
        (*stbuf).st_nlink = 1;
        (*stbuf).st_size = node.size();
    }

    println!("ending fuse_getattr");
    0
}

pub unsafe extern "C" fn fuse_open(_path: *const c_char, _fi: *mut raw::fuse_file_info) -> c_int {
    println!("hello_open called");
    0
}

pub unsafe extern "C" fn fuse_read(
    _path: *const c_char,
    buf: *mut c_char,
    _size: size_t,
    _offset: off_t,
    _fi: *mut raw::fuse_file_info,
) -> c_int {
    println!("fuse_read called");
    let node = FILES
        .as_ref()
        .unwrap()
        .data
        .search(CStr::from_ptr(_path).to_str().unwrap())
        .unwrap();
    if let Node::File(f) = node {
        memcpy(
            buf as *mut c_void,
            CString::new(f.data()).unwrap().as_ptr() as *mut c_void,
            f.data().len().try_into().unwrap(),
        );

        f.data().len().try_into().unwrap()
    } else {
        panic!()
    }
}

pub fn get_oper() -> raw::fuse_operations {
    return raw::fuse_operations {
        getattr: Some(fuse_getattr),
        readlink: None,
        mknod: None,
        mkdir: None,
        unlink: None,
        rmdir: None,
        symlink: None,
        rename: None,
        link: None,
        chmod: None,
        chown: None,
        truncate: None,
        open: Some(fuse_open),
        read: Some(fuse_read),
        write: None,
        statfs: None,
        flush: None,
        release: None,
        fsync: None,
        setxattr: None,
        getxattr: None,
        listxattr: None,
        removexattr: None,
        opendir: None,
        readdir: Some(fuse_readdir),
        releasedir: None,
        fsyncdir: None,
        init: Some(fuse_init),
        destroy: None,
        access: None,
        create: None,
        lock: None,
        utimens: None,
        bmap: None,
        ioctl: None,
        poll: None,
        write_buf: None,
        read_buf: None,
        flock: None,
        fallocate: None,
        copy_file_range: None,
    };
}
