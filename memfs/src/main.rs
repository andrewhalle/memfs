use fuse::FsBuilder;

mod file;
mod functions;

fn main() {
    // TODO build fs
    let fs = FsBuilder::new().finish();

    // start fs process
    fs.serve();

    // TODO cleanup
}
