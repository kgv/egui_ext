use egui::collapsing_header::CollapsingState;

/// Extension methods for [`CollapsingState`]
pub trait CollapsingStateExt {
    fn open(self, open: Option<bool>) -> Self;
}

impl CollapsingStateExt for CollapsingState {
    fn open(mut self, open: Option<bool>) -> Self {
        if let Some(open) = open {
            self.set_open(open);
        }
        self
    }
}
