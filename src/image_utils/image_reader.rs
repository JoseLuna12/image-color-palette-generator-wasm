use std::io::{Cursor, Read, Seek};

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat, Rgba};
use wasm_bindgen::prelude::wasm_bindgen;

use super::image_palette::ImagePalette;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct ImageInformation {
    pub persentage: u32,
    pub dimensions: (u32, u32),
    pub extra_y_space: u32,
    pub xspace_stepper: u32,
}

pub struct WorkingImage {
    information: ImageInformation,
    image: DynamicImage,
    result: Option<DynamicImage>,
}

impl WorkingImage {
    pub fn new(buffer: &[u8]) -> Self {
        let image = WorkingImage::load_image_from_buffer(buffer);
        let information = WorkingImage::get_information_from_image(&image);

        WorkingImage {
            image,
            information,
            result: None,
        }
    }

    pub fn get_information_from_image(image: &DynamicImage) -> ImageInformation {
        let (width, height) = image.dimensions();
        let default_percentage = 25;

        ImageInformation {
            persentage: default_percentage,
            dimensions: (width, height),
            extra_y_space: (default_percentage * height) / 100,
            xspace_stepper: width / (10),
        }
    }

    pub fn load_image_from_buffer(buffer: &[u8]) -> DynamicImage {
        match image::load_from_memory(buffer) {
            Ok(img) => img,
            Err(error) => {
                log("There was a problem opening the file");
                panic!("There was a problem opening the file: {:?}", error)
            }
        }
    }

    pub fn load_color_palette(&self) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let palette_colors = ImagePalette::new(&self.image);
        let palette_images = palette_colors.generate_palette_images(&self.information);
        palette_images
    }

    pub fn copy_image(&mut self) {
        let mut new_img = DynamicImage::new_rgb8(
            self.information.dimensions.0,
            self.information.dimensions.1 + self.information.extra_y_space,
        );
        match new_img.copy_from(&self.image, 0, 0) {
            Ok(_) => log("Success creating new image"),
            Err(e) => log(&format!("erro creating image {}", e)),
        };

        self.result = Some(new_img);
    }

    pub fn merge_palette_with_image(&mut self) -> Vec<u8> {
        let palette = self.load_color_palette();
        self.copy_image();

        let result = self.result.clone();

        if let Some(mut new_image) = result {
            let mut pos = 0;
            for p in palette {
                match new_image.copy_from(&p, pos, self.information.dimensions.1) {
                    Ok(_) => log("square added"),
                    Err(err) => log(&format!("error adding square {}", err)),
                }
                pos = pos + self.information.xspace_stepper;
            }
            self.result = Some(new_image);
        }

        self.transform_image_to_unit8()
    }

    pub fn transform_image_to_unit8(&self) -> Vec<u8> {
        let mut memory_cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        if let Some(image) = &self.result {
            match image.write_to(&mut memory_cursor, ImageFormat::Jpeg) {
                Ok(c) => c,
                Err(err) => {
                    log(&format!(
                        "There was a problem writing the resulting buffer {}",
                        err
                    ));
                    panic!(
                        "There was a problem writing the resulting buffer: {:?}",
                        err
                    )
                }
            }

            memory_cursor.seek(std::io::SeekFrom::Start(0)).unwrap();
            let mut out: Vec<u8> = Vec::new();
            memory_cursor.read_to_end(&mut out).unwrap();
            out
        } else {
            log("Empty result");
            panic!()
        }
    }
}
