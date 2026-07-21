pub mod collapsing_button;
pub mod collapsing_state;
pub mod color;
#[cfg(target_arch = "wasm32")]
pub mod download;
pub mod dropped_file;
pub mod hovered_file;
pub mod label;
pub mod labeled_separator;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod response;
pub mod spawn;
pub mod ui;
pub mod widgets;

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
