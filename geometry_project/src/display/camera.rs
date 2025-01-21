use crate::entities::extents2d::Extents2d;
use data_structures::vec2d::Vec2D;
use super::rgb::RGB;

pub struct Camera {
    pub view_port: Extents2d,
    buffer: Vec2D<RGB>
}

impl Camera
{
    pub fn new(view_port: Extents2d, buffer: Vec2D<RGB>) -> Self
    {
        Camera
        {
            view_port,
            buffer
        }
    }

    pub fn new_square(size: usize, width: f32) -> Self
    {
        let pixel: RGB = RGB::black();
        let view_port: Extents2d = Extents2d::new_square(width);
        let buffer: Vec2D<RGB> = Vec2D::new_size_reference(size, size, &pixel);

        Camera
        {
            view_port,
            buffer
        }

    }
}
