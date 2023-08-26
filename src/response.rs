use egui::{InnerResponse, Response};

/// Extension methods for [`InnerResponse`]
pub trait InnerResponseExt {
    fn flatten(self) -> Response;
}

impl InnerResponseExt for InnerResponse<Response> {
    fn flatten(self) -> Response {
        self.inner | self.response
    }
}
