pub use self::{
    collapsing_button::CollapsingButton,
    collapsing_state::CollapsingStateExt,
    color::color,
    dropped_file::DroppedFileExt,
    hovered_file::HoveredFileExt,
    label::ClickedLabel,
    labeled_separator::LabeledSeparator,
    response::{InnerResponseExt, ResponseExt},
    ui::{Doi, LightDarkButton},
};

#[cfg(target_arch = "wasm32")]
pub use download::download;
#[cfg(feature = "markdown")]
pub use markdown::Markdown;

pub mod prelude {
    pub use crate::{
        dropped_file::DroppedFileExt,
        hovered_file::HoveredFileExt,
        label::ClickedLabel,
        labeled_separator::LabeledSeparator,
        response::{InnerResponseExt as _, ResponseExt as _},
        ui::{Doi as _, LightDarkButton as _},
    };

    #[cfg(feature = "markdown")]
    pub use crate::markdown::Markdown;
}

pub mod ui;
pub mod widgets;

#[cfg(target_arch = "wasm32")]
pub mod download;

mod collapsing_button;
mod collapsing_state;
mod color;
mod dropped_file;
mod hovered_file;
mod label;
mod labeled_separator;
mod response;

#[cfg(feature = "markdown")]
mod markdown;
