# MemFS

## prelim

* want to create an in-memory filesystem that survives as long as the program runs
* use FUSE
    * implement open, read, write, stat, etc.

## data structures

* file, directory

## 2019-10-12

* got basic functionality working
* need to write a safe wrapper for fuse
* S_ISDIR and S_ISREG (from sys/stat.h) are super important (that's what was causing my errors)
* #! for crate level allows
* goal: all filesystem functions should be safe. only unsafe code should be in fuse bindings crate

## 2019-10-21

project organization

memfs
├── Cargo.toml
├── fuse-sys
│   ├── lib.rs
│   └── raw.rs
└── memfs
    ├── file.rs
    ├── functions.rs
    └── main.rs

* memfs workspace
    * fuse-sys crate
        * raw module, with the bindings
        * lib, expose the safe wrapper (FsBuilder)
    * memfs crate
        * file module, implementing the data structures required for the filesystem
        * functions module, implementing the functions for the file system
        * main, tie everything together

## 2019-11-02

decided on a trait/trait object model for communicating between fuse-sys and memfs, filled out most
of the implementation, and it builds, but unable to read the directory, because of an apparent double
free? weird. Also, moved away from using fuse_main, so will be able to easily implement catching
CTRL-C to stop the process and unmount, when I get it working.

moving away from fuse_main has made debugging much easier, because I can use the normal println!,
rather than writing to a file. The problem causing the double free is the unwrap of the shared
option, possibly because unwrapping the option causes what's inside it to be dropped at the end of
scope? not 100% sure yet

double free was caused by the CString! I was treating the path pointer as owned memory and turning
it into a CString. When that went out of scope, free was called. Switching to use CStr fixed the
issue.
