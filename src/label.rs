use egui::{Label, Response, RichText, Sense, Ui, WidgetText};

/// Extension methods for [`Ui`]
pub trait ClickedLabel {
    fn clicked_label(&mut self, text: impl Into<WidgetText>) -> Response;

    fn clicked_heading(&mut self, text: impl Into<RichText>) -> Response;
}

impl ClickedLabel for Ui {
    fn clicked_label(&mut self, text: impl Into<WidgetText>) -> Response {
        self.add(Label::new(text).sense(Sense::click()))
    }

    fn clicked_heading(&mut self, text: impl Into<RichText>) -> Response {
        self.add(Label::new(text.into().heading()).sense(Sense::click()))
    }
}
