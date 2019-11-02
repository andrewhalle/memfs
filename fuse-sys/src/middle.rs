use libc::{memcpy, memset, off_t, size_t, S_IFDIR, S_IFREG};
use std::convert::TryInto;
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null_mut;

use super::raw;
use crate::Fs;

pub static mut FILES: Option<Fs> = None;

fn log(s: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/memfs-log.txt")
        .unwrap();
    file.write_all("rust: ".as_bytes()).unwrap();
    file.write_all(s.as_bytes()).unwrap();
    file.write_all("\n".as_bytes()).unwrap();
}

pub unsafe extern "C" fn fuse_init(
    _conn: *mut raw::fuse_conn_info,
    cfg: *mut raw::fuse_config,
) -> *mut c_void {
    log("hello_init called");
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
    log("hello_readdir called");
    filler.unwrap()(buf, CString::new(".").unwrap().as_ptr(), null_mut(), 0, 0);
    filler.unwrap()(buf, CString::new("..").unwrap().as_ptr(), null_mut(), 0, 0);
    let dir = FILES
        .as_ref()
        .unwrap()
        .data
        .getdir(
            &CString::from_raw(_path as *mut c_char)
                .into_string()
                .unwrap(),
        )
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
    log("hello_getattr called");
    memset(stbuf as *mut c_void, 0, size_of::<raw::stat>());
    let node = FILES
        .as_ref()
        .unwrap()
        .data
        .search(
            &CString::from_raw(path as *mut c_char)
                .into_string()
                .unwrap(),
        )
        .unwrap();
    if node.is_directory() {
        (*stbuf).st_mode = 0o755 | S_IFDIR;
        (*stbuf).st_nlink = 2;
    } else {
        (*stbuf).st_mode = 0o755 | S_IFREG;
        (*stbuf).st_nlink = 1;
        (*stbuf).st_size = node.size();
    }

    0
}

pub unsafe extern "C" fn fuse_open(_path: *const c_char, _fi: *mut raw::fuse_file_info) -> c_int {
    log("hello_open called");
    0
}

pub unsafe extern "C" fn fuse_read(
    _path: *const c_char,
    buf: *mut c_char,
    _size: size_t,
    _offset: off_t,
    _fi: *mut raw::fuse_file_info,
) -> c_int {
    let length: i32 = 13;
    memcpy(
        buf as *mut c_void,
        CString::new("hello world\n".as_bytes()).unwrap().as_ptr() as *mut c_void,
        length.try_into().unwrap(),
    );
    length
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
