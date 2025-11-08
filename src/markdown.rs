use egui::{Color32, Id, Image, ImageSource, Ui, load::Bytes, mutex::Mutex};
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use std::{collections::HashMap, sync::Arc};

/// Extension methods for [`Ui`]
///
/// `ui.markdown("$X_2$");`
pub trait Markdown {
    fn markdown(&mut self, markdown: &str);
}

impl Markdown for Ui {
    fn markdown(&mut self, markdown: &str) {
        let cache = (
            self.data_mut(|data| {
                data.get_temp_mut_or_default::<Arc<Mutex<HashMap<_, _>>>>(Id::new(
                    "GlobalMathCache",
                ))
                .clone()
            }),
            self.data_mut(|data| {
                data.get_temp_mut_or_default::<Arc<Mutex<CommonMarkCache>>>(Id::new(
                    "GlobalMarkdownCache",
                ))
                .clone()
            }),
        );
        CommonMarkViewer::new()
            .render_math_fn(Some(&move |ui, math, inline| {
                let mut cache = cache.0.lock();
                let svg = cache
                    .entry(math.to_string())
                    .or_insert_with(|| render_math(math, inline));
                let uri = format!("{}.svg", Id::from(math.to_string()).value());
                ui.add(
                    Image::new(ImageSource::Bytes {
                        uri: uri.into(),
                        bytes: Bytes::Shared(svg.clone()),
                    })
                    .tint(ui.visuals().strong_text_color())
                    .fit_to_original_size(1.0),
                );
            }))
            .show(self, &mut cache.1.lock(), markdown);
    }
}

fn render_math(math: &str, inline: bool) -> Arc<[u8]> {
    if inline {
        mathjax_svg::convert_to_svg_inline(math).unwrap()
    } else {
        mathjax_svg::convert_to_svg(math).unwrap()
    }
    .replace("currentColor", "white")
    .into_bytes()
    .into()
}
