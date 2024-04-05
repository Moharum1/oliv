use std::ops::Mul;

use crate::engine::math::matrix::Matrix4X4;
use crate::engine::math::vector::CoOrdinate;
use crate::engine::math::vector::CoOrdinateType::Vector;

impl CoOrdinate{
    pub fn translate(self, x:f32, y:f32, z:f32) -> CoOrdinate{
        let translate_mat = Matrix4X4 {
            rows: [
                [1.0, 0.0 , 0.0,  x],
                [0.0, 1.0, 0.0,   y],
                [0.0, 0.0, 1.0,   z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        translate_mat * self
    }

    pub fn inverse_translate(self, x:f32, y:f32, z:f32)-> CoOrdinate{
        let mut translate_mat = Matrix4X4 {
            rows: [
                [1.0, 0.0 , 0.0,  x],
                [0.0, 1.0, 0.0,   y],
                [0.0, 0.0, 1.0,   z],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        translate_mat = translate_mat.inverse().unwrap();

        translate_mat * self
    }
}

pub struct Translation(pub(crate) Matrix4X4);

impl Translation{
    pub fn new(x:f32, y:f32, z:f32) -> Translation {
        Translation{
            0: Matrix4X4 {
                rows: [
                    [1.0, 0.0 , 0.0,  x],
                    [0.0, 1.0, 0.0,   y],
                    [0.0, 0.0, 1.0,   z],
                    [0.0, 0.0, 0.0, 1.0]
                ]
            },
        }
    }

    pub fn inverse(mut self) -> Translation {
        let inverse = self.0.inverse();

        if let Some(data) = inverse{
            self.0 = data;
        }

        self
    }
}

impl Mul<CoOrdinate> for Translation {
    type Output = CoOrdinate;

    fn mul(self, rhs: CoOrdinate) -> Self::Output {

        let type_value = (rhs.kind as i32) as f32;

        let x = {
            self.0.rows[0][0] * rhs.x +
                self.0.rows[0][1] * rhs.y +
                self.0.rows[0][2] * rhs.z +
                self.0.rows[0][3] * type_value
        };

        let y = {
                self.0.rows[1][0] * rhs.x +
                self.0.rows[1][1] * rhs.y +
                self.0.rows[1][2] * rhs.z +
                self.0.rows[1][3] * type_value
        };

        let z = {
                self.0.rows[2][0] * rhs.x +
                self.0.rows[2][1] * rhs.y +
                self.0.rows[2][2] * rhs.z +
                self.0.rows[2][3] * type_value
        };


        if rhs.kind == Vector{
            CoOrdinate::new_vector(x,y,z)
        }else {
            CoOrdinate::new_point(x,y,z)
        }
    }
}

#[cfg(test)]
mod test{
    use crate::engine::math::vector::CoOrdinate;

    #[test]
    fn transform_with_points(){
        let point = CoOrdinate::new_point(-3.0, 4.0, 5.0);

        assert_eq!(point.translate(5.0, -3.0, 2.0), CoOrdinate::new_point(2.0, 1.0, 7.0))
    }

    #[test]
    fn transform_with_vectors(){
        let point = CoOrdinate::new_vector(-3.0, 4.0, 5.0);

        assert_eq!(point.translate(5.0, -3.0, 2.0),CoOrdinate::new_vector(-3.0, 4.0, 5.0))
    }

    #[test]
    fn transform_with_inverse_matrix(){
        let point = CoOrdinate::new_point(-3.0, 4.0, 5.0);

        assert_eq!(point.inverse_translate(5.0, -3.0, 2.0), CoOrdinate::new_point(-8.0, 7.0, 3.0))
    }
}

