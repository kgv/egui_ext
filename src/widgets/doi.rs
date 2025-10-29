use egui::{Response, RichText, Ui};

pub fn doi(doi: &str) -> impl Fn(&mut Ui) -> Response {
    move |ui| {
        ui.hyperlink_to(
            RichText::new(format!("DOI: {doi}")).heading(),
            format!("https://doi.org/{doi}"),
        )
    }
}
