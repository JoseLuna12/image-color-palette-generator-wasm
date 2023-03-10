use std::io::{Cursor, Read, Seek};

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat, Rgba};
use wasm_bindgen::prelude::wasm_bindgen;

use super::{defaults::Defaults, image_palette::ImagePalette};

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
    pub extension: ImageFormat,
}

pub struct WorkingImage {
    defaults: Defaults,
    information: ImageInformation,
    image: DynamicImage,
    result: Option<DynamicImage>,
}

impl WorkingImage {
    pub fn new(buffer: &[u8], extension: &str, defaults: Defaults) -> Self {
        let format = match extension {
            "jpeg" | "jpg" => ImageFormat::Jpeg,
            "png" => ImageFormat::Png,
            _ => {
                log("image not supported");
                panic!("Image Not supported");
            }
        };

        let image = WorkingImage::load_image_from_buffer(buffer, format);
        let information = WorkingImage::get_information_from_image(&image, format, &defaults);

        WorkingImage {
            defaults,
            image,
            information,
            result: None,
        }
    }

    pub fn get_information_from_image(
        image: &DynamicImage,
        extension: ImageFormat,
        defaults: &Defaults,
    ) -> ImageInformation {
        let (width, height) = image.dimensions();

        ImageInformation {
            persentage: defaults.default_y_space.get(),
            dimensions: (width, height),
            extra_y_space: (defaults.default_y_space.get() * height) / 100,
            xspace_stepper: width / (defaults.palette_quantity.get() - 1),
            extension,
        }
    }

    pub fn load_image_from_buffer(buffer: &[u8], format: ImageFormat) -> DynamicImage {
        match image::load_from_memory_with_format(buffer, format) {
            Ok(img) => img,
            Err(error) => {
                log(&format!("There was a problem opening the file {}", error));
                panic!("There was a problem opening the file: {:?}", error)
            }
        }
    }

    pub fn load_color_palette(&self) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let palette_colors = ImagePalette::new(&self.image, &self.defaults, &self.information);
        let palette_images = palette_colors.generate_palette_images(&self.information);
        palette_images
    }

    pub fn copy_image(&mut self) {
        let mut new_img = match self.information.extension {
            ImageFormat::Png => DynamicImage::new_rgb8(
                self.information.dimensions.0,
                self.information.dimensions.1 + self.information.extra_y_space,
            ),
            ImageFormat::Jpeg => DynamicImage::new_rgb8(
                self.information.dimensions.0,
                self.information.dimensions.1 + self.information.extra_y_space,
            ),
            _ => DynamicImage::new_rgb8(
                self.information.dimensions.0,
                self.information.dimensions.1 + self.information.extra_y_space,
            ),
        };

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
            match image.write_to(&mut memory_cursor, self.information.extension) {
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
