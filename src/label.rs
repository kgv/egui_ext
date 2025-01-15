use egui::{Label, Response, Sense, Ui, WidgetText};

/// Extension methods for [`Ui`]
pub trait ClickedLabel {
    fn clicked_label(&mut self, text: impl Into<WidgetText>) -> Response;
}

impl ClickedLabel for Ui {
    fn clicked_label(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add(Label::new(text).sense(Sense::click()))
    }
}
