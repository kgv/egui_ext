use egui::{RichText, Ui, Visuals};
use egui_phosphor::regular::{MOON, SUN};

/// Extension methods for [`Ui`]
pub trait LightDarkButton {
    fn light_dark_button(&mut self, size: f32);
}

impl LightDarkButton for Ui {
    fn light_dark_button(&mut self, size: f32) {
        let style = (**self.style()).clone();
        if style.visuals.dark_mode {
            if self
                .button(RichText::new(SUN).size(size))
                .on_hover_text("Switch to light mode")
                .clicked()
            {
                self.ctx().set_visuals(Visuals::light());
            }
        } else {
            if self
                .button(RichText::new(MOON).size(size))
                .on_hover_text("Switch to dark mode")
                .clicked()
            {
                self.ctx().set_visuals(Visuals::dark());
            }
        }
    }
}
