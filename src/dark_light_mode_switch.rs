use crate::WithVisuals;
use egui::{Button, RichText, Ui, Visuals};

// TODO https://github.com/emilk/egui/issues/3051

/// Extension methods for [`Ui`]
pub trait DarkLightModeSwitch {
    fn heading_dark_light_mode_switch(&mut self, f: impl FnMut(Visuals) -> Visuals);
}

impl DarkLightModeSwitch for Ui {
    fn heading_dark_light_mode_switch(&mut self, mut f: impl FnMut(Visuals) -> Visuals) {
        self.with_visuals(|ui, visuals| {
            if visuals.dark_mode
                && ui
                    .add(Button::new(RichText::new("☀").heading()).frame(false))
                    .on_hover_text("Switch to light mode")
                    .clicked()
            {
                *visuals = f(Visuals::light());
            } else if !visuals.dark_mode
                && ui
                    .add(Button::new(RichText::new("🌙").heading()).frame(false))
                    .on_hover_text("Switch to dark mode")
                    .clicked()
            {
                *visuals = f(Visuals::dark());
            }
        });
    }
}
