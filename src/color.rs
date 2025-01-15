use eframe::epaint::Hsva;
use egui::Color32;

pub fn color(index: usize) -> Color32 {
    // 0.61803398875
    let golden_ratio: f32 = (5.0_f32.sqrt() - 1.0) / 2.0;
    let h = index as f32 * golden_ratio;
    Hsva::new(h, 0.85, 0.5, 1.0).into()
}
