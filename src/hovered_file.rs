use egui::HoveredFile;
use std::fmt::{self, Formatter};

/// Extension methods for [`HoveredFile`]
pub trait HoveredFileExt {
    fn display(&self) -> Display<'_>;
}

impl HoveredFileExt for HoveredFile {
    fn display(&self) -> Display<'_> {
        Display(self)
    }
}

/// Display
#[derive(Clone, Copy, Debug)]
pub struct Display<'a>(&'a HoveredFile);

impl fmt::Display for Display<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if let Some(path) = &self.0.path {
            write!(f, "{}", path.display())?;
        } else if !self.0.mime.is_empty() {
            write!(f, "{}", self.0.mime)?;
        } else {
            f.write_str("???")?;
        }
        Ok(())
    }
}
