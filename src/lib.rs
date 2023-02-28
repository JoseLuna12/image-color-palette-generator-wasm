mod image_utils;
use image_utils::{defaults::Defaults, image_reader::WorkingImage};
// use wasm_bindgen::prelude::wasm_bindgen;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// #[wasm_bindgen]
pub fn get_image_color_palette(
    unit8arr: &[u8],
    extension: &str,
    palette_size: Option<u32>,
) -> Vec<u8> {
    let defaults = match palette_size {
        Some(p) => Defaults::get_custom(p, 5, 15),
        None => Defaults::get(),
    };

    let mut img = image_utils::image_reader::WorkingImage::new(unit8arr, extension, defaults);
    img.merge_palette_with_image()
}

pub fn get_image_palette_file(path: String) {
    let defaults = Defaults::get_custom(21, 5, 15);
    //save_image_with_palette
    let mut img = WorkingImage::new_file(&path, defaults);
    img.save_image_with_palette("test.jpeg")
}
