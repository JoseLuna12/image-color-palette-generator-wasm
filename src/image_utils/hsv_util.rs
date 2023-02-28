use std::fmt::Display;

pub struct HsvValues {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

impl Display for HsvValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "h: {} s: {} v: {}", self.h, self.s, self.v)
    }
}

pub struct HSVColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub result: Option<HsvValues>,
    pub luminosity: Option<f32>,
}

impl HSVColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        let (h, s, v) = HSVColor::convert_rgb_to_hsv(r, g, b);
        let hsv = HsvValues { h, s, v };
        let mut prev_hsv = HSVColor {
            r,
            g,
            b,
            result: Some(hsv),
            luminosity: None,
        };
        let luminosity = prev_hsv.get_luminosity();
        prev_hsv.luminosity = Some(luminosity);
        prev_hsv
    }

    pub fn get_luminosity(&self) -> f32 {
        let r = self.r as f32 * 0.241;
        let g = self.g as f32 * 0.691;
        let b = self.b as f32 * 0.068;

        let sum = r + g + b;
        sum.sqrt()
    }

    pub fn calculate_hue_from_rgb(rgb: (f32, f32, f32), max: f32, diff: f32) -> f32 {
        let (new_r, new_g, new_b) = rgb;
        if diff == 0f32 {
            0f32
        } else if max == new_r {
            let calc = (new_g - new_b) / diff;
            let mod_val = calc % 6f32;
            60f32 * mod_val
        } else if max == new_g {
            let calc = (new_b.powf(diff) - new_r) / diff;
            let sum = calc + 2f32;
            60f32 * sum
        } else if max == new_b {
            let calc = (new_r.powf(diff) - new_g) / diff;
            let sum = calc + 4f32;
            60f32 * sum
        } else {
            0f32
        }
    }

    pub fn convert_rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
        let new_r = r as f32 / 255f32;
        let new_g = g as f32 / 255f32;
        let new_b = b as f32 / 255f32;

        let mut color_rgb = [new_r, new_g, new_b];
        color_rgb.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let max = match color_rgb.last() {
            Some(m) => *m,
            None => panic!(),
        };
        let min = color_rgb[0];

        let diff = max - min;

        let h_value = f32::trunc(HSVColor::calculate_hue_from_rgb(
            (new_r, new_g, new_b),
            max,
            diff,
        ));

        let raw_s_value = if max == 0f32 { 0f32 } else { diff / max };
        let s_value = f32::trunc(raw_s_value * 100f32) / 100f32;
        let v_value = f32::trunc(max * 100f32) / 100f32;

        (h_value, s_value, v_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_to_hsv() {
        let hsv = HSVColor::new(100, 94, 32);

        match hsv.result {
            Some(value) => {
                println!("{}", value);
                assert_eq!(value.h, 55f32);
            }
            None => assert!(true),
        }
    }
}
