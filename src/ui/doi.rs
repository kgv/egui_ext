use crate::widgets::doi;
use egui::{Response, Ui};

/// Extension methods for [`Ui`]
pub trait Doi {
    fn doi(&mut self, text: &str) -> Response;
}

impl Doi for Ui {
    fn doi(&mut self, text: &str) -> Response {
        doi(text)(self)
    }
}
