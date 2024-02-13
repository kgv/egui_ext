use super::InnerResponseExt;
use eframe::emath::Numeric;
use egui::{DragValue, Response, Ui};
use std::ops::Bound;

/// Extension methods for [`Ui`]
pub trait UiExt {
    fn drag_bound<T: Numeric>(
        &mut self,
        bound: &mut Bound<T>,
        f: impl FnMut(DragValue) -> DragValue,
    ) -> Response;

    fn drag_option<T: Numeric>(
        &mut self,
        value: &mut Option<T>,
        f: impl FnMut(DragValue) -> DragValue,
    ) -> Response;

    // fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response;
}

// impl UiExt for Ui {
//     fn drag_bound<T: Numeric>(
//         &mut self,
//         bound: &mut Bound<T>,
//         mut f: impl FnMut(DragValue) -> DragValue,
//     ) -> Response {
//         let unbounded = matches!(bound, Bound::Unbounded);
//         self.add_enabled_ui(!unbounded, |ui| {
//             let mut nan = T::from_f64(f64::NAN);
//             let value = match bound {
//                 Bound::Included(value) | Bound::Excluded(value) => value,
//                 Bound::Unbounded => &mut nan,
//             };
//             let mut drag_value = DragValue::new(value);
//             if unbounded {
//                 drag_value = drag_value.custom_formatter(|_, _| "∞".to_owned());
//             }
//             ui.add(f(drag_value))
//         })
//         .flatten()
//         .on_hover_text(match bound {
//             Bound::Included(_) => "Included",
//             Bound::Excluded(_) => "Excluded",
//             Bound::Unbounded => "Unbounded",
//         })
//         .context_menu(|ui| {
//             let included = matches!(bound, Bound::Included(_));
//             if !included && ui.selectable_label(included, "Included").clicked() {
//                 *bound = Bound::Included(if let Bound::Excluded(value) = bound {
//                     *value
//                 } else {
//                     T::from_f64(f64::default())
//                 });
//                 ui.close_menu();
//             };
//             let excluded = matches!(bound, Bound::Excluded(_));
//             if !excluded && ui.selectable_label(excluded, "Excluded").clicked() {
//                 *bound = Bound::Excluded(if let Bound::Included(value) = bound {
//                     *value
//                 } else {
//                     T::from_f64(f64::default())
//                 });
//                 ui.close_menu();
//             };
//             if !unbounded && ui.selectable_label(unbounded, "Unbounded").clicked() {
//                 *bound = Bound::Unbounded;
//                 ui.close_menu();
//             };
//         })
//     }

//     fn drag_option<T: Numeric>(
//         &mut self,
//         option: &mut Option<T>,
//         mut f: impl FnMut(DragValue) -> DragValue,
//     ) -> Response {
//         let mut default = T::from_f64(f64::default());
//         let none = option.is_none();
//         self.add_enabled_ui(!none, |ui| {
//             let mut drag_value = DragValue::new(option.as_mut().unwrap_or(&mut default));
//             if none {
//                 drag_value = drag_value.custom_formatter(|_, _| "-".to_owned());
//             }
//             ui.add(f(drag_value))
//         })
//         .flatten()
//         .on_hover_text(match option {
//             None => "None",
//             Some(_) => "Some",
//         })
//         .context_menu(|ui| {
//             if !none && ui.button("None").clicked() {
//                 *option = None;
//                 ui.close_menu();
//             } else if none && ui.button("Some").clicked() {
//                 *option = Some(default);
//                 ui.close_menu();
//             }
//         })
//     }

//     // fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response {
//     //     DragValue::new(value)
//     //         .clamp_range(0..=100)
//     //         .speed(0.1)
//     //         .suffix('%')
//     //         .ui(self)
//     // }
// }
