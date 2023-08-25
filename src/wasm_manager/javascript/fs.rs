use std::{io, path::Path};

use wasm_bindgen::prelude::*;

fn err_js_to_io(err: JsValue) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        err.as_string().unwrap_or(String::new()),
    )
}

#[cfg(feature = "browser")]
pub fn save_buffer(path: impl AsRef<Path>, buf: impl AsRef<[u8]>) -> io::Result<()> {
    // If we are in a browser...
    use js_sys::{Array, Uint8Array};
    use std::ffi::OsStr;
    use web_sys::Url;
    fn inner(path: impl AsRef<Path>, buf: impl AsRef<[u8]>) -> Result<(), JsValue> {
        // ... get the filename (the rest of the path is irrelevant)...
        let path = path
            .as_ref()
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        // ... create an <a></a> tag...
        let doc = web_sys::window().unwrap().document().unwrap();
        let body = doc.body().unwrap();
        let anchor: JsValue = doc.create_element("a")?.into();
        let anchor: web_sys::HtmlAnchorElement = anchor.into();

        // ... transform the buffer to a blob url...
        let buf: Uint8Array = buf.as_ref().into();
        let buf = Array::of1(&buf);
        let file = web_sys::Blob::new_with_u8_array_sequence(&buf)?;
        let url = Url::create_object_url_with_blob(&file)?;

        // ... then use that url to download the buffer
        anchor.set_href(&url);
        anchor.set_download(path);
        body.append_child(&anchor)?;
        anchor.click();
        body.remove_child(&anchor)?;
        Url::revoke_object_url(&url)?;

        Ok(())
    }

    inner(path, buf).map_err(err_js_to_io)
}

#[cfg(feature = "nodejs")]
pub fn save_buffer(path: impl AsRef<Path>, buf: impl AsRef<[u8]>) -> io::Result<()> {
    #[wasm_bindgen]
    extern "C" {
        fn require(_: &str) -> JsValue;
    }

    let write_file_sync =
        js_sys::Reflect::get(&require("fs"), &JsValue::from_str("writeFileSync")).unwrap();
    let write_file_sync: js_sys::Function = write_file_sync.into();

    node_deno_save_buffer(path, buf, |path, buf| {
        write_file_sync.call2(&JsValue::NULL, &JsValue::from_str(path), &buf)?;
        Ok(())
    })
}

#[cfg(feature = "deno")]
pub fn save_buffer(path: impl AsRef<Path>, buf: impl AsRef<[u8]>) -> io::Result<()> {
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = Deno)]
        fn writeFileSync(_: &str, _: js_sys::Uint8Array);
    }

    node_deno_save_buffer(path, buf, |path, buf| {
        writeFileSync(path, buf);
        Ok(())
    })
}

#[cfg(any(feature = "nodejs", feature = "deno"))]
fn node_deno_save_buffer(
    path: impl AsRef<Path>,
    buf: impl AsRef<[u8]>,
    write_file_sync: impl Fn(&str, js_sys::Uint8Array) -> Result<(), JsValue>,
) -> io::Result<()> {
    let buf: js_sys::Uint8Array = buf.as_ref().into();
    let path = path.as_ref().to_str().ok_or(io::Error::new(
        io::ErrorKind::InvalidInput,
        "non-UTF-8 path",
    ))?;

    write_file_sync(path, buf).map_err(err_js_to_io)
}
