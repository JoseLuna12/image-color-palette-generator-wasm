pub struct DefaultPaletteQuantity(u32);

impl DefaultPaletteQuantity {
    pub fn new(quantity: u32) -> Self {
        DefaultPaletteQuantity(quantity)
    }

    pub fn get(&self) -> u32 {
        *&self.0
    }
}
pub struct DefaultPaletteBorder(u32);

impl DefaultPaletteBorder {
    pub fn new(quantity: u32) -> Self {
        DefaultPaletteBorder(quantity)
    }
    pub fn get(&self) -> u32 {
        *&self.0
    }
}

pub struct DefaultYSpace(u32);

impl DefaultYSpace {
    pub fn new(quantity: u32) -> Self {
        DefaultYSpace(quantity)
    }
    pub fn get(&self) -> u32 {
        *&self.0
    }
}

trait GetVal {
    fn get(&self) -> u32;
}

pub struct Defaults {
    pub palette_quantity: DefaultPaletteQuantity,
    pub palette_border: DefaultPaletteBorder,
    pub default_y_space: DefaultYSpace,
}

impl Defaults {
    pub fn get() -> Self {
        let palette_quantity = DefaultPaletteQuantity::new(21);
        let palette_border = DefaultPaletteBorder::new(5);
        let default_y_space = DefaultYSpace::new(15);

        Defaults {
            palette_quantity,
            palette_border,
            default_y_space,
        }
    }

    pub fn get_custom(palette: u32, border: u32, y_space: u32) -> Self {
        let palette_quantity = DefaultPaletteQuantity::new(palette);
        let palette_border = DefaultPaletteBorder::new(border);
        let default_y_space = DefaultYSpace::new(y_space);

        Defaults {
            palette_quantity,
            palette_border,
            default_y_space,
        }
    }
}
