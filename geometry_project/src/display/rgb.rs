use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct RGB {
    pub value: u32,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        RGB {
            value: (u32::from(red) << 16) | (u32::from(green) << 8) | (u32::from(blue) << 0),
        }
    }

    pub fn black() -> Self {
        RGB { value: 0u32 }
    }

    pub fn white() -> Self {
        RGB { value: 0xfffffff }
    }

    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_red() {
        let red: RGB = RGB::red();
        let expected: u32 = 0xff0000;

        assert_eq!(expected, red.value);
    }
}
