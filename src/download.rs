#[cfg(target_arch = "wasm32")]
pub use self::web::download;

#[cfg(target_arch = "wasm32")]
mod web {
    use gloo_utils::errors::JsError;
    use js_sys::{Array, Uint8Array};
    use wasm_bindgen::prelude::*;
    use web_sys::{File, FilePropertyBag, HtmlAnchorElement, Url, window};

    /// * https://stackoverflow.com/questions/3665115/how-to-create-a-file-in-memory-for-user-to-download-but-not-through-server
    /// * https://stackoverflow.com/questions/69556755/web-sysurlcreate-object-url-with-blobblob-not-formatting-binary-data-co
    /// * https://github.com/emilk/egui/discussions/3571
    pub fn download(name: &str, bytes: &[u8]) -> Result<(), String> {
        || -> Result<(), JsValue> {
            let window = window().expect("window not found");
            let document = window.document().expect("document not found");

            let output: HtmlAnchorElement = document.create_element("a")?.dyn_into()?;
            output.style().set_property("display", "none")?;
            output.set_href(&file(name, bytes)?);
            output.set_download(name);
            output.click();
            Ok(())
        }()
        .map_err(|error| match JsError::try_from(error) {
            Ok(error) => error.to_string(),
            Err(error) => error.to_string(),
        })
    }

    fn file(name: &str, bytes: &[u8]) -> Result<String, JsValue> {
        let array = Uint8Array::from(bytes);
        let bits = Array::new();
        bits.push(&array.buffer());
        let options = FilePropertyBag::new();
        options.set_type("application/octet-stream");
        let file = File::new_with_blob_sequence_and_options(&bits, name, &options)?;
        Ok(Url::create_object_url_with_blob(&file)?)
    }
}
