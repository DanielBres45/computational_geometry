#[derive(Clone, Copy)]
pub struct RGB {
    pub value: u32,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        RGB {
            value: (u32::from(red) << 16) & (u32::from(green) << 8) & (u32::from(blue) << 0),
        }
    }

    pub fn black() -> Self {
        Self::new(255u8, 255u8, 255u8)
    }

    pub fn white() -> Self {
        RGB { value: 0xfffffff }
    }
}
