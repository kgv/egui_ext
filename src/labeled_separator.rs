use egui::{Response, Separator, Ui, WidgetText};

/// Extension methods for [`Ui`]
pub trait LabeledSeparator {
    fn labeled_separator(&mut self, text: impl Into<WidgetText>) -> Response;
}

impl LabeledSeparator for Ui {
    fn labeled_separator(&mut self, text: impl Into<WidgetText>) -> Response {
        self.horizontal(|ui| {
            ui.label(text);
            ui.add(Separator::default().horizontal());
        })
        .response
    }
}
