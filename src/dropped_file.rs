use egui::DroppedFile;
use std::{
    fmt::{self, Formatter},
    fs::{read, read_to_string},
    io,
    string::FromUtf8Error,
};
use thiserror::Error;

/// Extension methods for [`DroppedFile`]
pub trait DroppedFileExt {
    fn bytes(&self) -> Result<Vec<u8>>;

    fn content(&self) -> Result<String>;

    fn display(&self) -> Display;

    fn name(&self) -> &str;
}

impl DroppedFileExt for DroppedFile {
    fn bytes(&self) -> Result<Vec<u8>> {
        Ok(match &self.bytes {
            Some(bytes) => bytes.to_vec(),
            None => match &self.path {
                Some(path) => read(path)?,
                None => return Err(Error::BytesOrPathNotFoud),
            },
        })
    }

    fn content(&self) -> Result<String> {
        Ok(String::from_utf8(self.bytes()?)?)
    }

    fn display(&self) -> Display {
        Display(self)
    }

    fn name(&self) -> &str {
        if let Some(name) = &self
            .path
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|file_name| file_name.to_str())
        {
            name
        } else if !self.name.is_empty() {
            &self.name
        } else {
            "???"
        }
    }
}

/// Display
#[derive(Clone, Copy, Debug)]
pub struct Display<'a>(&'a DroppedFile);

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(path) = &self.0.path {
            write!(f, "{}", path.display())?;
        } else if !self.0.name.is_empty() {
            write!(f, "{}", self.0.name)?;
        } else {
            f.write_str("???")?;
        };
        if let Some(bytes) = &self.0.bytes {
            write!(f, " ({} bytes)", bytes.len()).ok();
        }
        Ok(())
    }
}

/// Error
#[derive(Debug, Error)]
pub enum Error {
    #[error("dropped file hasn't bytes or path")]
    BytesOrPathNotFoud,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Utf8(#[from] FromUtf8Error),
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
