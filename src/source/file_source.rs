use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

use super::Source;

struct BinaryFileSource {
    file: File,
}

impl BinaryFileSource {
    fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(Self { file })
    }
}

impl Source<Result<Vec<u8>>> for BinaryFileSource {
    fn read(&mut self) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        self.file.read_to_end(&mut data)?;
        Ok(data)
    }
}

struct TextFileSource {
    file: File,
}

impl TextFileSource {
    fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file = File::open(path)?;
        Ok(Self { file })
    }
}

impl Source<Result<String>> for TextFileSource {
    fn read(&mut self) -> Result<String> {
        let mut data = String::new();
        self.file.read_to_string(&mut data)?;
        Ok(data)
    }
}
