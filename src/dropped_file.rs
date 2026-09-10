use egui::DroppedFile;
use std::{
    fmt::{self, Formatter},
    io,
    string::FromUtf8Error,
};
use thiserror::Error;

/// Extension methods for [`DroppedFile`]
pub trait DroppedFileExt {
    // fn bytes(&self) -> Result<Vec<u8>>;

    fn content(&self) -> Result<String>;

    fn display(&self) -> Display<&Self>;

    fn name(&self) -> &str;
}

impl<T: DroppedFile> DroppedFileExt for T {
    fn content(&self) -> Result<String> {
        Ok(String::from_utf8(self.bytes().unwrap())?)
    }

    fn display(&self) -> Display<&Self> {
        Display(self)
    }

    fn name(&self) -> &str {
        if let Some(name) = &self
            .path()
            .file_name()
            .and_then(|file_name| file_name.to_str())
        {
            name
        } else {
            "???"
        }
    }
}

/// Display
#[derive(Clone, Copy, Debug)]
pub struct Display<T>(T);

impl<T: DroppedFile> fmt::Display for Display<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0.path().display())?;
        if let Ok(bytes) = &self.0.bytes() {
            write!(f, " ({} bytes)", bytes.len()).ok();
        }
        // if let Some(path) = &self.0.path {
        //     write!(f, "{}", path.display())?;
        // } else if !self.0.name.is_empty() {
        //     write!(f, "{}", self.0.name)?;
        // } else {
        //     f.write_str("???")?;
        // };
        // if let Some(bytes) = &self.0.bytes {
        //     write!(f, " ({} bytes)", bytes.len()).ok();
        // }
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
    // #[error("{_0}")]
    // Bytes(#[from] String),
}

/// Result
pub type Result<T> = std::result::Result<T, Error>;
