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
