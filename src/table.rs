use egui::{Separator, Ui};
use egui_extras::{TableBody, TableRow};

/// Extension methods for [`TableBody`]
pub trait TableBodyExt {
    fn separate(&mut self, height: f32, columns: usize);
}

impl TableBodyExt for TableBody<'_> {
    fn separate(&mut self, height: f32, columns: usize) {
        self.row(height, |mut row| {
            for _ in 0..columns {
                row.col(|ui| {
                    ui.add(Separator::default().horizontal());
                });
            }
        });
    }
}

/// Extension methods for [`TableRow`]
pub trait TableRowExt {
    fn cols(&mut self, count: usize, add_cell_contents: impl Fn(&mut Ui));
}

impl TableRowExt for TableRow<'_, '_> {
    fn cols(&mut self, count: usize, add_cell_contents: impl Fn(&mut Ui)) {
        for _ in 0..count {
            self.col(&add_cell_contents);
        }
    }
}
