use egui::{Ui, Visuals};

/// Extension methods for [`Ui`]
pub trait WithVisuals {
    fn with_visuals(&mut self, f: impl FnMut(&mut Self, &mut Visuals));
}

impl WithVisuals for Ui {
    fn with_visuals(&mut self, mut f: impl FnMut(&mut Self, &mut Visuals)) {
        let mut visuals = self.ctx().style().visuals.clone();
        f(self, &mut visuals);
        self.ctx().set_visuals(visuals);
    }
}
