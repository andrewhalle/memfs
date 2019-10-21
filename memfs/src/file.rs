pub struct File {
    data: Vec<u8>,
}

impl File {
    pub fn new(init_data: &[u8]) -> File {
        let mut data = Vec::with_capacity(init_data.len());
        for &b in init_data.iter() {
            data.push(b);
        }

        File {
            data
        }
    }
}

#[cfg(test)]
mod tests {
    use super::File;

    #[test]
    fn it_works() {
        let _f = File::new("hello, world!\n".as_bytes());
    }
}
