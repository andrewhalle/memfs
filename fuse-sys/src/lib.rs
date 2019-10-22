mod raw;

pub struct Fs {}

impl Fs {
    pub fn serve(self) {}
}

pub struct FsBuilder {}

impl FsBuilder {
    pub fn new() -> FsBuilder {
        FsBuilder {}
    }

    pub fn finish(self) -> Fs {
        Fs {}
    }
}
