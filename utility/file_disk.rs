use std::fs::File;
use std::io::{Error, Result, Read, Write, Seek, SeekFrom};

use redoxfs::Disk;

pub struct FileDisk {
    path: String,
    file: File
}

impl FileDisk {
    pub fn new(path: &str) -> Result<FileDisk> {
        let file = try!(File::open(path));
        Ok(FileDisk {
            path: path.to_string(),
            file: file
        })
    }
}

impl Disk<Error> for FileDisk {
    fn name(&self) -> &str {
        &self.path
    }

    fn read_at(&mut self, block: u64, buffer: &mut [u8]) -> Result<usize> {
        try!(self.file.seek(SeekFrom::Start(block * 512)));
        self.file.read(buffer)
    }

    fn write_at(&mut self, block: u64, buffer: &[u8]) -> Result<usize> {
        try!(self.file.seek(SeekFrom::Start(block * 512)));
        self.file.write(buffer)
    }
}