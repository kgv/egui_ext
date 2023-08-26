use super::{InnerResponseExt, ResponseExt};
// use crate::utils::BoundExt;
use eframe::emath::Numeric;
use egui::{DragValue, Response, Ui, Widget};
use serde::{Deserialize, Serialize};
use std::ops::Bound;

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn drag_bound<T>(
        &mut self,
        bound: &mut Bound<T>,
        f: impl FnMut(DragValue) -> DragValue,
    ) -> Response
    where
        for<'a> T: Numeric + Serialize + Deserialize<'a> + Send + Sync;

    fn drag_option<T: Numeric>(
        &mut self,
        value: &mut Option<T>,
        f: impl FnMut(DragValue) -> DragValue,
    ) -> Response;

    // fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response;
}

impl UiExt for Ui {
    fn drag_bound<T>(
        &mut self,
        bound: &mut Bound<T>,
        mut f: impl FnMut(DragValue) -> DragValue,
    ) -> Response
    where
        for<'a> T: Numeric + Serialize + Deserialize<'a> + Send + Sync,
    {
        let id = self.id().with("value");
        let mut value = bound.value().copied().unwrap_or_else(|| {
            self.data_mut(|data| data.get_persisted(id).unwrap_or(T::from_f64(f64::NAN)))
        });
        match bound {
            Bound::Unbounded => self
                .add_enabled_ui(false, |ui| {
                    ui.add(f(
                        DragValue::new(&mut value).custom_formatter(|_, _| "∞".to_owned())
                    ))
                })
                .flatten(),
            Bound::Included(value) | Bound::Excluded(value) => self.add(f(DragValue::new(value))),
        }
        .on_hover_text({
            match bound {
                Bound::Included(_) => "Included",
                Bound::Excluded(_) => "Excluded",
                Bound::Unbounded => "Unbounded",
            }
        })
        .context_menu(|ui| {
            let response = ui.selectable_value(bound, Bound::Included(value), "Included")
                | ui.selectable_value(bound, Bound::Excluded(value), "Excluded")
                | ui.selectable_value(bound, Bound::Unbounded, "Unbounded")
                    .with_clicked(|| {
                        self.data_mut(|data| data.insert_persisted(id, value));
                    });
            if response.clicked() {
                ui.close_menu();
            }
            // if ui.ui_contains_pointer() && ui.input(|input| input.pointer.any_click()) {
            //     ui.close_menu();
            // }
        })
    }

    fn drag_option<T: Numeric>(
        &mut self,
        value: &mut Option<T>,
        mut f: impl FnMut(DragValue) -> DragValue,
    ) -> Response {
        let mut default = T::from_f64(0.0);
        let enabled = value.is_some();
        self.add_enabled_ui(enabled, |ui| {
            ui.add(
                f(DragValue::new(value.as_mut().unwrap_or(&mut default))).custom_formatter(
                    |value, _| {
                        if enabled {
                            value.to_string()
                        } else {
                            "-".to_owned()
                        }
                    },
                ),
            )
        })
        .flatten()
        .on_hover_text({
            match value {
                None => "None",
                Some(_) => "Some",
            }
        })
        .context_menu(|ui| {
            if enabled && ui.button("None").clicked() {
                *value = None;
                ui.close_menu();
            } else if !enabled && ui.button("Some").clicked() {
                *value = Some(default);
                ui.close_menu();
            }
        })
    }

    // fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response {
    //     DragValue::new(value)
    //         .clamp_range(0..=100)
    //         .speed(0.1)
    //         .suffix('%')
    //         .ui(self)
    // }
}
