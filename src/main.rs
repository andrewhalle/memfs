use std::ffi::CString;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null_mut;

pub mod fuse;

unsafe extern "C" fn hello_init(
    _conn: *mut fuse::fuse_conn_info,
    cfg: *mut fuse::fuse_config,
) -> *mut c_void {
    (*cfg).kernel_cache = 1;
    null_mut()
}

unsafe extern "C" fn hello_readdir(
    path: *const c_char,
    buf: *mut c_void,
    filler: fuse::fuse_fill_dir_t,
    _offset: fuse::off_t,
    _fi: *mut fuse::fuse_file_info,
    _flags: fuse::fuse_readdir_flags,
) -> c_int {
    println!("readdir called");
    filler.unwrap()(buf, CString::new(".").unwrap().as_ptr(), null_mut(), 0, 0);
    filler.unwrap()(buf, CString::new("..").unwrap().as_ptr(), null_mut(), 0, 0);
    filler.unwrap()(
        buf,
        CString::new("hello.txt").unwrap().as_ptr(),
        null_mut(),
        0,
        0,
    );

    0
}

unsafe extern "C" fn hello_getattr(
    path: *const c_char,
    stbuf: *mut fuse::stat,
    fi: *mut fuse::fuse_file_info,
) -> c_int {
    (*stbuf).st_mode = 0o755;
    (*stbuf).st_nlink = 2;
    0
}

unsafe extern "C" fn hello_open(path: *const c_char, fi: *mut fuse::fuse_file_info) -> c_int {
    0
}

fn main() {
    let mut hello_oper = fuse::fuse_operations {
        getattr: Some(hello_getattr),
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
        open: Some(hello_open),
        read: None,
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
        readdir: Some(hello_readdir),
        releasedir: None,
        fsyncdir: None,
        init: Some(hello_init),
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

    let argc = 2;
    let mut argv = [
        CString::new("memfs").unwrap().into_raw(),
        CString::new("/home/andrew/Desktop/memfs")
            .unwrap()
            .into_raw(),
    ];

    unsafe {
        fuse::fuse_main_real(
            argc,
            &mut argv[0] as *mut *mut c_char,
            &mut hello_oper as *mut fuse::fuse_operations,
            size_of::<fuse::fuse_operations>(),
            null_mut(),
        );
    }
}
