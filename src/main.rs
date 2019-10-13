use libc::{memcpy, memset, off_t, size_t, strcmp, S_IFDIR, S_IFREG};
use std::ffi::CString;
use std::fs::OpenOptions;
use std::io::Write;
use std::mem::size_of;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null_mut;

pub mod fuse;

fn log(s: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/home/andrew/Desktop/log.txt")
        .unwrap();
    file.write_all("rust: ".as_bytes());
    file.write_all(s.as_bytes());
    file.write_all("\n".as_bytes());
}

unsafe extern "C" fn hello_init(
    _conn: *mut fuse::fuse_conn_info,
    cfg: *mut fuse::fuse_config,
) -> *mut c_void {
    log("hello_init called");
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
    log("hello_readdir called");
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
    log("hello_getattr called");
    memset(stbuf as *mut c_void, 0, size_of::<fuse::stat>());
    if strcmp(path, CString::new("/").unwrap().as_ptr()) == 0 {
        (*stbuf).st_mode = 0o755 | S_IFDIR;
        (*stbuf).st_nlink = 2;
    } else {
        (*stbuf).st_mode = 0o755 | S_IFREG;
        (*stbuf).st_nlink = 1;
        (*stbuf).st_size = 13;
    }
    0
}

unsafe extern "C" fn hello_open(path: *const c_char, fi: *mut fuse::fuse_file_info) -> c_int {
    log("hello_open called");
    0
}

unsafe extern "C" fn hello_read(
    path: *const c_char,
    buf: *mut c_char,
    size: size_t,
    offset: off_t,
    fi: *mut fuse::fuse_file_info,
) -> c_int {
    memcpy(
        buf as *mut c_void,
        CString::new("hello world\n").unwrap().as_ptr() as *mut c_void,
        13,
    );
    13
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
        read: Some(hello_read),
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
