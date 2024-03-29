use std::ops;

#[derive(PartialEq,Debug,Clone)]
pub(crate) struct Color{
    r: f32,
    g: f32,
    b: f32
}

impl Color{
    pub(crate) fn new(r:f32, g:f32, b:f32) -> Color {
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


#[cfg(test)]
mod test{
    use crate::engine::colors::Color;

    // Note : All Basic operations works but with a floating point error

    #[test]
    fn check_color_addition(){
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 + c2;

        assert_eq!(res, Color::new(1.6, 0.7, 1.0))
    }

    #[test]
    fn check_color_sub(){
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 - c2;

        assert_eq!(res, Color::new(0.2, 0.5, 0.5))
    }

    #[test]
    fn check_color_mul_with_scalar(){
        let c1 = Color::new(0.2, 0.3, 0.4);
        let c2 = c1 * 2;

        assert_eq!(c2, Color::new(0.4, 0.6, 0.8))
    }

    #[test]
    fn check_color_mul_with_color(){
        let c1 = Color::new(1f32, 0.2, 0.4);
        let c2 = Color::new(0.9, 1f32, 0.1);
        let res = c1 * c2;

        assert_eq!(res, Color::new(0.9, 0.2, 0.04))
    }
}