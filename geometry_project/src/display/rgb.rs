use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct RGB(pub u32);

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for RGB {
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        RGB((u32::from(red) << 16) | (u32::from(green) << 8) | (u32::from(blue) << 0))
    }

    pub fn black() -> Self {
        RGB(0u32)
    }

    pub fn white() -> Self {
        RGB(0xfffffff)
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

        assert_eq!(expected, red.0);
    }
}
