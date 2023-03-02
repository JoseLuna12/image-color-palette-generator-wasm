use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba};
use palette_extract::{
    get_palette_with_options, Color, MaxColors, PixelEncoding, PixelFilter, Quality,
};

use super::{defaults::Defaults, hsv_util::HSVColor, image_reader::ImageInformation};

pub struct ImagePalette {
    pub palette: Vec<Color>,
    pub default_border: u32,
}

pub enum BlackThreshold {
    Low,
    Med,
    High,
}

impl ImagePalette {
    pub fn new(image: &DynamicImage, defaults: &Defaults, information: &ImageInformation) -> Self {
        let palette = ImagePalette::get_palette(
            image,
            defaults.palette_quantity.get(),
            information.extension,
        );
        let filtered = ImagePalette::filter_black_color(palette, defaults);
        let mut reordered_palette = ImagePalette::rearrange_pallete(filtered);

        while (reordered_palette.len() as u32) > defaults.palette_quantity.get() - 1 {
            reordered_palette.pop();
        }

        ImagePalette {
            palette: reordered_palette,
            default_border: defaults.palette_border.get(),
        }
    }

    pub fn get_palette(
        image: &DynamicImage,
        max_colors: u32,
        encode_information: ImageFormat,
    ) -> Vec<Color> {
        let pixels = image.as_bytes();

        let encoding = match encode_information {
            ImageFormat::Png => PixelEncoding::Rgba,
            ImageFormat::Jpeg => PixelEncoding::Rgb,
            _ => PixelEncoding::Rgb,
        };

        get_palette_with_options(
            pixels,
            encoding,
            Quality::new(1),
            MaxColors::new(max_colors as u8 + 10),
            PixelFilter::None,
        )
    }

    pub fn filter_black_color(palette: Vec<Color>, defaults: &Defaults) -> Vec<Color> {
        // let re_arranged_palette = ImagePalette::rearrange_pallete(palette);
        let mut new_filtered: Vec<Color> = Vec::new();
        let mut rejected: Vec<Color> = Vec::new();

        let threshold = match defaults.black_threshold {
            BlackThreshold::Low => 3f32,
            BlackThreshold::Med => 4f32,
            BlackThreshold::High => 5f32,
        };

        for pal in palette {
            let r = pal.r;
            let g = pal.g;
            let b = pal.b;

            let hsv = HSVColor::new(r, g, b);
            if let Some(lum) = hsv.luminosity {
                if lum > threshold {
                    new_filtered.push(pal);
                } else {
                    rejected.push(pal)
                }
            }
        }

        while (new_filtered.len() as u32) < defaults.palette_quantity.get() - 1 {
            if let Some(rej_color) = rejected.iter().next() {
                new_filtered.push(*rej_color)
            };
        }
        new_filtered
    }

    fn rearrange_pallete(palette: Vec<Color>) -> Vec<Color> {
        let mut new_palette: Vec<Color> = Vec::from(palette);
        let repetitions = 8f32;

        new_palette.sort_by(|a, b| {
            let hsv_color_a = HSVColor::new(a.r, a.g, a.b);
            let hsv_color_b = HSVColor::new(b.r, b.g, b.b);

            let get_compare_values = |hsv_val: HSVColor| {
                let lum = match hsv_val.luminosity {
                    Some(l) => l,
                    None => panic!(),
                };

                match hsv_val.result {
                    Some(hsv) => {
                        let h2 = hsv.h * repetitions;
                        let mut lum2 = lum * repetitions;
                        let mut v2 = hsv.v * repetitions;

                        if h2 % 2f32 == 1f32 {
                            v2 = repetitions - v2;
                            lum2 = repetitions - lum;
                        }

                        return (h2, lum2, v2);
                    }
                    None => panic!(""),
                }
            };

            let a_compare = get_compare_values(hsv_color_a);
            let b_compare = get_compare_values(hsv_color_b);

            b_compare.partial_cmp(&a_compare).unwrap()
        });
        new_palette
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

//https://www.alanzucconi.com/2015/09/30/colour-sorting/
