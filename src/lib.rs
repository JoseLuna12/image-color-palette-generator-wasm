mod image_utils;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn aver(unit8arr: &[u8]) -> Vec<u8> {
    let mut img = image_utils::image_reader::WorkingImage::new(unit8arr);
    img.merge_palette_with_image()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         // aver()
//         // let result = add(2, 2);
//         // assert_eq!(result, 4);
//     }
// }
