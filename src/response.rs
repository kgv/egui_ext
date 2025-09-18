use egui::{InnerResponse, Response, Tooltip, Ui};

/// Extension methods for [`InnerResponse`]
pub trait InnerResponseExt<T, E> {
    fn transpose(self) -> Result<Option<InnerResponse<T>>, E>;
}

impl<T, E> InnerResponseExt<T, E> for Option<InnerResponse<Result<T, E>>> {
    fn transpose(self) -> Result<Option<InnerResponse<T>>, E> {
        match self {
            Some(InnerResponse { inner, response }) => Ok(Some(InnerResponse {
                inner: inner?,
                response,
            })),
            None => Ok(None),
        }
    }
}

/// Extension methods for [`Response`]
pub trait ResponseExt: Sized {
    fn try_on_hover_ui<E>(
        self,
        add_contents: impl Fn(&mut Ui) -> Result<(), E>,
    ) -> Result<Self, E> {
        Ok(self
            .try_on_disabled_hover_ui(&add_contents)?
            .try_on_enabled_hover_ui(&add_contents)?)
    }

    fn try_on_enabled_hover_ui<E>(
        self,
        add_contents: impl FnOnce(&mut Ui) -> Result<(), E>,
    ) -> Result<Self, E>;

    fn try_on_disabled_hover_ui<E>(
        self,
        add_contents: impl FnOnce(&mut Ui) -> Result<(), E>,
    ) -> Result<Self, E>;
}

impl ResponseExt for Response {
    fn try_on_enabled_hover_ui<E>(
        self,
        add_contents: impl FnOnce(&mut Ui) -> Result<(), E>,
    ) -> Result<Self, E> {
        if let Some(inner_response) = Tooltip::for_enabled(&self).show(add_contents) {
            inner_response.inner?;
        }
        Ok(self)
    }

    fn try_on_disabled_hover_ui<E>(
        self,
        add_contents: impl FnOnce(&mut Ui) -> Result<(), E>,
    ) -> Result<Self, E> {
        if let Some(inner_response) = Tooltip::for_disabled(&self).show(add_contents) {
            inner_response.inner?;
        }
        Ok(self)
    }
}
