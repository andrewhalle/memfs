mod raw;

pub struct Fs {}

pub struct FsBuilder {}

impl FsBuilder {
    pub fn new() -> FsBuilder {
        FsBuilder {}
    }

    pub fn finish(self) -> Fs {
        Fs {}
    }
}
