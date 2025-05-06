pub use self::{
    collapsing_button::CollapsingButton, collapsing_state::CollapsingStateExt, color::color,
    dark_light_mode_switch::DarkLightModeSwitch, dropped_file::DroppedFileExt,
    hovered_file::HoveredFileExt, label::ClickedLabel, labeled_separator::LabeledSeparator,
    light_dark_button::LightDarkButton, response::InnerResponseExt, with_visuals::WithVisuals,
};

#[cfg(target_arch = "wasm32")]
pub use download::download;

mod collapsing_button;
mod collapsing_state;
mod color;
mod dark_light_mode_switch;
#[cfg(target_arch = "wasm32")]
pub mod download;
mod dropped_file;
mod hovered_file;
mod label;
mod labeled_separator;
mod light_dark_button;
mod response;
mod ui;
mod with_visuals;
