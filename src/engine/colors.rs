use std::ops;

#[derive(PartialEq,Debug,Clone)]
pub(crate) struct Color{
    r: f32,
    g: f32,
    b: f32
}

impl Color{
    pub(crate) const fn new(r:f32, g:f32, b:f32) -> Color {
        Color{
            r,g,b
        }
    }

    pub(crate) fn to_u8(&self) -> [u8; 3] {
        [
            self.r as u8,
            self.g as u8,
            self.b as u8,
        ]
    }
}

impl ops::Add for Color{
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color{
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Sub for Color{
    type Output = Color;

    fn sub(self, rhs: Self) -> Self::Output {
        Color{
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::Mul<i32> for Color{
    type Output = Color;

    fn mul(self, rhs: i32) -> Self::Output {
        Self{
            r: self.r * rhs as f32,
            g: self.g * rhs as f32,
            b: self.b * rhs as f32,
        }
    }
}

impl ops::Mul for Color{
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}
