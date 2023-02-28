use image::{DynamicImage, ImageBuffer, Rgba};
use palette_extract::{
    get_palette_with_options, Color, MaxColors, PixelEncoding, PixelFilter, Quality,
};

use super::image_reader::ImageInformation;

pub struct ImagePalette {
    pub palette: Vec<Color>,
    pub default_border: u32,
}

impl ImagePalette {
    pub fn new(image: &DynamicImage) -> Self {
        let palette = ImagePalette::get_palette(image);
        ImagePalette {
            palette,
            default_border: 5,
        }
    }

    pub fn get_palette(image: &DynamicImage) -> Vec<Color> {
        let pixels = image.as_bytes();
        get_palette_with_options(
            pixels,
            PixelEncoding::Rgb,
            Quality::new(1),
            MaxColors::new(11),
            PixelFilter::None,
        )
    }

    pub fn generate_palette_images(
        &self,
        image_information: &ImageInformation,
    ) -> Vec<ImageBuffer<Rgba<u8>, Vec<u8>>> {
        let (width, _) = image_information.dimensions;
        let single_color_palette_width = image_information.xspace_stepper;
        let single_color_palette_height = image_information.extra_y_space;

        self.palette
            .iter()
            .enumerate()
            .map(|(index, color)| {
                let magic_width = get_magic_width(
                    (width, single_color_palette_width),
                    index,
                    self.palette.len(),
                );

                ImageBuffer::from_fn(magic_width, single_color_palette_height, |x, y| {
                    get_pallete_square_color(
                        color,
                        self.default_border,
                        magic_width,
                        single_color_palette_height,
                        (x, y),
                    )
                })
            })
            .collect()
    }
}

pub fn get_magic_width(w_dimension: (u32, u32), index: usize, length: usize) -> u32 {
    let (total_w, color_w) = w_dimension;

    let mut actual_width = color_w;

    let diff = color_w * (length as u32);

    if index + 1 == length {
        if diff < total_w {
            let to_sum = total_w - diff;
            actual_width = color_w + to_sum;
        }
    }
    actual_width
}

fn get_pallete_square_color(
    color: &Color,
    border: u32,
    width: u32,
    height: u32,
    coordinates: (u32, u32),
) -> Rgba<u8> {
    let (x, y) = coordinates;
    let square_color = Rgba([color.r, color.g, color.b, 0]);
    let white: Rgba<u8> = Rgba([255, 255, 255, 0]);
    if x < border || x > width - border {
        white
    } else if y < border || y > height - border {
        white
    } else {
        square_color
    }
}
