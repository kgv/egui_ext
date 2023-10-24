use egui::{
    collapsing_header::paint_default_icon, pos2, Button, Id, InnerResponse, Response, Ui, Vec2,
    WidgetText,
};
use std::hash::Hash;

/// Collapsing button
#[derive(Clone)]
pub struct CollapsingButton {
    id_source: Id,
    text: WidgetText,
    open: Option<bool>,
}

impl CollapsingButton {
    pub fn new(text: impl Into<WidgetText>) -> Self {
        let text = text.into();
        let id_source = Id::new(text.text());
        Self {
            id_source,
            text,
            open: None,
        }
    }

    pub fn id_source(mut self, id_source: impl Hash) -> Self {
        self.id_source = Id::new(id_source);
        self
    }

    pub fn open(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    pub fn show(self, ui: &mut Ui) -> InnerResponse<bool> {
        let Self {
            id_source: id,
            text,
            open,
        } = self;
        let id = ui.make_persistent_id(id);
        let mut open = ui.data_mut(|data| match open {
            Some(open) => {
                data.insert_temp(id, open);
                open
            }
            None => *data.get_temp_mut_or_default(id),
        });
        let min_size = Vec2::splat(ui.spacing().icon_width);
        let response = ui.add(Button::new(text).min_size(min_size));
        response.icon(ui, id, open);
        if response.clicked() {
            open ^= true;
            ui.data_mut(|data| data.insert_temp(id, open));
        }
        InnerResponse::new(open, response)
    }
}

/// Icon
trait Icon {
    fn icon(&self, ui: &mut Ui, id: Id, open: bool);
}

impl Icon for Response {
    fn icon(&self, ui: &mut Ui, id: Id, open: bool) {
        let mut rect = ui.spacing().icon_rectangles(self.rect).0;
        if self.rect.aspect_ratio() == 1.0 {
            rect.set_center(pos2(
                self.rect.left() + ui.spacing().indent / 2.0,
                self.rect.center().y,
            ));
        }
        let response = self.clone().with_new_rect(rect);
        let openness = ui.ctx().animate_bool(id, open);
        paint_default_icon(ui, openness, &response);
    }
}
