use egui::{Id, Image, ImageSource, Rgba, Ui, load::Bytes, mutex::Mutex};
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
                let color = ui.visuals().strong_text_color();
                let svg = cache
                    .entry(math.to_string())
                    .or_insert_with(|| render_math(math, inline, color.into()));
                let uri = format!("{}.svg", Id::from(math.to_string()).value());
                ui.add(
                    Image::new(ImageSource::Bytes {
                        uri: uri.into(),
                        bytes: Bytes::Shared(svg.clone()),
                    })
                    .fit_to_original_size(1.0),
                );
            }))
            .show(self, &mut cache.1.lock(), markdown);
    }
}

fn render_math(math: &str, inline: bool, color: Rgba) -> Arc<[u8]> {
    use ratex_layout::{LayoutOptions, layout, to_display_list};
    use ratex_parser::parser::parse;
    use ratex_svg::{SvgOptions, render_to_svg};
    use ratex_types::color::Color;

    let layout_opts = LayoutOptions {
        color: Color {
            r: color.r(),
            g: color.g(),
            b: color.b(),
            a: color.a(),
        },
        ..Default::default()
    };
    let svg_opts = SvgOptions {
        font_size: 20.0,
        // padding: 10.0,
        // stroke_width: 1.5,
        embed_glyphs: true,
        ..Default::default()
    };
    if inline {
        let ast = parse(math).unwrap();
        let layout = layout(&ast, &layout_opts);
        let display_list = to_display_list(&layout);
        render_to_svg(&display_list, &svg_opts)
    } else {
        let ast = parse(math).unwrap();
        let layout = layout(&ast, &layout_opts);
        let display_list = to_display_list(&layout);
        render_to_svg(&display_list, &svg_opts)
    }
    .into_bytes()
    .into()
}
