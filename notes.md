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
