use std::{
    fmt::{Debug, Display},
    io,
};

use rust_xlsxwriter::{Workbook, XlsxError};

mod wasm_manager;
use wasm_manager::*;

#[cfg_attr(
    all(
        target_arch = "wasm32",
        not(any(target_os = "emscripten", target_os = "wasi"))
    ),
    wasm_bindgen::prelude::wasm_bindgen(start)
)]
pub fn start() -> Result<(), Error> {
    std::panic::set_hook(Box::new(|panic_info| log::log(&panic_info.to_string())));

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.write_string(1, 1, "WASM Hello World!!")?;
    worksheet.write_string(3, 1, "This is an example")?;

    let buf = workbook.save_to_buffer()?;
    fs::save_buffer("hello-world-example.xlsx", buf)?;

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Xlsx(XlsxError),
    IO(io::Error),
}

impl From<XlsxError> for Error {
    fn from(value: XlsxError) -> Self {
        Self::Xlsx(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
