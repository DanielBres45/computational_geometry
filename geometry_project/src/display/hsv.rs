use crate::{display::rgb::RGB, entities::angle::Angle};

pub struct HSV
{
    pub hue: Angle,
    pub saturation: f32,
    pub value: f32
}

pub const PHI: f64 = 1.618033988749894848204586834365638118_f64; //golden ratio
pub const GOLDEN_CONJUGATE: f64 = 0.618033988749895_f64;

impl HSV {

    pub fn new(hue: Angle, saturation: f32, value: f32) -> Self
    {
        HSV { hue, saturation, value }
    }

    pub fn to_rgb(&self) -> RGB {
        let h = self.hue.degrees() % 360.0;
        let s = self.saturation.clamp(0.0, 1.0);
        let v = self.value.clamp(0.0, 1.0);
        
        let c = v * s; // Chroma
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs() as f32);
        let m = v - c;
        
        let (r_prime, g_prime, b_prime) = match h {
            h if h < 60.0 => (c, x, 0.0),
            h if h < 120.0 => (x, c, 0.0),
            h if h < 180.0 => (0.0, c, x),
            h if h < 240.0 => (0.0, x, c),
            h if h < 300.0 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
        
        let r = ((r_prime + m) * 255.0).round() as u8;
        let g = ((g_prime + m) * 255.0).round() as u8;
        let b = ((b_prime + m) * 255.0).round() as u8;
        
        RGB::new(r, g, b)
    }

    pub fn random_color(saturation: f32, value: f32) -> HSV {
        let hue =  Angle(rand::random_range(0f64..2f64));
        HSV 
        {
            hue,
            saturation,
            value
        }
    }

    pub fn random_colors(count: usize, saturation: f32, value: f32) -> Vec<HSV> {
        let mut hue = Angle(rand::random_range(0f64..2f64));
        let mut colors: Vec<HSV> = Vec::with_capacity(count);

        for _ in 0..count
        {
            hue = hue + Angle(GOLDEN_CONJUGATE);
            hue = hue.modulo(&Angle(1f64));

            colors.push(HSV{hue, saturation, value});
        }

        return colors;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hsv_to_rgb_1() {
        let hsv: HSV = HSV::new(Angle::from_degrees(5f64), 0.80, 0.80);
        let rgb: RGB = RGB::new(204, 54, 41);

        assert_eq!(rgb, hsv.to_rgb());
    }

}