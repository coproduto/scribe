use std::fs;
use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;

use super::Source;

struct BinaryFileSource<'a, PathType> {
    path: &'a PathType,
    file: File,
}

impl<'a, PathType> BinaryFileSource<'a, PathType>
where
    PathType: AsRef<Path>,
{
    fn new(path: &'a PathType) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self { path, file })
    }
}

impl<'a, PathType> Source<Result<(fs::Metadata, Vec<u8>)>> for BinaryFileSource<'a, PathType>
where
    PathType: AsRef<Path>,
{
    fn read(&mut self) -> Result<(fs::Metadata, Vec<u8>)> {
        let metadata = fs::metadata(self.path)?;
        let mut data = Vec::new();
        self.file.read_to_end(&mut data)?;
        Ok((metadata, data))
    }
}

struct TextFileSource<'a, PathType> {
    path: &'a PathType,
    file: File,
}

impl<'a, PathType> TextFileSource<'a, PathType>
where
    PathType: AsRef<Path>,
{
    fn new(path: &'a PathType) -> Result<Self> {
        let file = File::open(path)?;
        Ok(Self { path, file })
    }
}

impl<'a, PathType> Source<Result<(fs::Metadata, String)>> for TextFileSource<'a, PathType>
where
    PathType: AsRef<Path>,
{
    fn read(&mut self) -> Result<(fs::Metadata, String)> {
        let metadata = fs::metadata(self.path)?;
        let mut data = String::new();
        self.file.read_to_string(&mut data)?;
        Ok((metadata, data))
    }
}
