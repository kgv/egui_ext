use egui::{Align, Layout, Rect, Response, Separator, Ui};
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

    fn right_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response);

    fn left_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response);
}

impl TableRowExt for TableRow<'_, '_> {
    fn cols(&mut self, count: usize, add_cell_contents: impl Fn(&mut Ui)) {
        for _ in 0..count {
            self.col(&add_cell_contents);
        }
    }

    fn right_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response) {
        self.col(|ui| {
            ui.with_layout(
                Layout::left_to_right(Align::Center)
                    .with_main_align(Align::RIGHT)
                    .with_main_justify(true),
                content,
            );
        })
    }

    fn left_align_col<R>(&mut self, content: impl FnOnce(&mut Ui) -> R) -> (Rect, Response) {
        self.col(|ui| {
            ui.with_layout(
                Layout::left_to_right(Align::Center)
                    .with_main_align(Align::LEFT)
                    .with_main_justify(true),
                content,
            );
        })
    }
}
