#[cfg(target_arch = "wasm32")]
pub use self::web::{DOCX, NONE, XLS, XLSX, download};

#[cfg(target_arch = "wasm32")]
mod web {
    use js_sys::{Array, Uint8Array};
    #[cfg(feature = "tracing")]
    use tracing::instrument;
    use wasm_bindgen::prelude::*;
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url, window};

    pub const NONE: &str = "application/octet-stream";
    pub const DOCX: &str =
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
    pub const XLS: &str = "application/vnd.ms-excel";
    pub const XLSX: &str = "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";

    // * https://stackoverflow.com/questions/3665115/how-to-create-a-file-in-memory-for-user-to-download-but-not-through-server
    // * https://stackoverflow.com/questions/69556755/web-sysurlcreate-object-url-with-blobblob-not-formatting-binary-data-co
    // * https://github.com/emilk/egui/discussions/3571
    // * https://github.com/ippras-utca/utca/blob/bca91021413c4089f412d07267147db097c94eb6/src/widgets/file_dialog/mod.rs
    #[cfg_attr(feature = "tracing", instrument(err(Debug)))]
    pub fn download(bytes: &[u8], r#type: &str, name: &str) -> Result<(), JsValue> {
        let Some(window) = window() else {
            return Err(JsError::new("window is none").into());
        };
        let Some(document) = window.document() else {
            return Err(JsError::new("document is none").into());
        };
        let a = document.create_element("a")?;
        let link = HtmlAnchorElement::unchecked_from_js_ref(&a);
        link.style().set_property("display", "none")?;
        link.set_download(name);
        link.set_href(&url(bytes, r#type)?);
        link.click();
        Ok(())
    }

    fn url(bytes: &[u8], r#type: &str) -> Result<String, JsValue> {
        let bytes = Uint8Array::from(bytes);
        let array = Array::new();
        array.push(&bytes.buffer());
        let options = BlobPropertyBag::new();
        options.set_type(r#type);
        let blob = Blob::new_with_u8_array_sequence_and_options(&array, &options)?;
        Url::create_object_url_with_blob(&blob)
    }

    // fn file(name: &str, bytes: &[u8], r#type: &str) -> Result<String, JsValue> {
    //     let array = Uint8Array::from(bytes);
    //     let bits = Array::new();
    //     bits.push(&array.buffer());
    //     let options = FilePropertyBag::new();
    //     options.set_type(r#type);
    //     let file = File::new_with_blob_sequence_and_options(&bits, name, &options)?;
    //     Ok(Url::create_object_url_with_blob(&file)?)
    // }
}
