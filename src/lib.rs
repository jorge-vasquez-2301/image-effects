use base64::Engine;
use image::ImageFormat;
use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded_file)
        .unwrap();
    log(&"Image decoded".into());

    let img = image::load_from_memory(&bytes).unwrap();
    log(&"Image loaded".into());

    let img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = Vec::new();
    img.write_to(&mut buffer, ImageFormat::Png).unwrap();
    log(&"New image written".into());

    let encoded_image = base64::engine::general_purpose::STANDARD.encode(&buffer);
    format!("data:image/png;base64,{encoded_image}")
}
