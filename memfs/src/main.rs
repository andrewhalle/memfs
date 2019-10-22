use fuse::FsBuilder;

mod file;
mod functions;

fn main() {
    let fs = FsBuilder::new().finish();
}
