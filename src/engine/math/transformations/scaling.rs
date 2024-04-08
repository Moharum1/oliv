use std::ops::Mul;
use crate::engine::math::matrix::Matrix4X4;
use crate::engine::math::transformations::MatTransform::Translation;
use crate::engine::math::vector::CoOrdinate;

impl CoOrdinate{

    // TODO: Implement the transpose trait for the func like Translation
    pub fn scale(self, x:f32, y:f32, z:f32) -> CoOrdinate{
        let scale_mat = Matrix4X4 {
            rows: [
                [x  , 0.0, 0.0,   0.0],
                [0.0, y  , 0.0,   0.0],
                [0.0, 0.0, z  ,   0.0],
                [0.0, 0.0, 0.0,   1.0]
            ]
        };

        scale_mat * self
    }

    // TODO: Implement the transpose trait for the func like Translation
    pub fn shrink(self, x:f32, y:f32, z:f32) -> CoOrdinate{
        let mut scale_mat = Matrix4X4 {
            rows: [
                [x  , 0.0, 0.0,   0.0],
                [0.0, y  , 0.0,   0.0],
                [0.0, 0.0, z  ,   0.0],
                [0.0, 0.0, 0.0,   1.0]
            ]
        };

        scale_mat = scale_mat.inverse().unwrap();

        scale_mat * self
    }

    // TODO: Implement the transpose trait for the func like Translation
    pub fn reflection(self, x:f32, y:f32, z:f32) -> CoOrdinate{
        let scale_mat = Matrix4X4 {
            rows: [
                [x  , 0.0, 0.0,   0.0],
                [0.0, y  , 0.0,   0.0],
                [0.0, 0.0, z  ,   0.0],
                [0.0, 0.0, 0.0,   1.0]
            ]
        };

        scale_mat * self
    }
}

pub(crate) struct Scale(pub Matrix4X4);

impl Scale{
    pub fn new(x:f32, y:f32, z:f32) -> Scale {
        Scale{
            0: Matrix4X4 {
                rows: [
                    [x  , 0.0, 0.0,   0.0],
                    [0.0, y  , 0.0,   0.0],
                    [0.0, 0.0, z  ,   0.0],
                    [0.0, 0.0, 0.0,   1.0]
                ]
            }
        }
    }

    pub fn inverse(mut self) -> Scale {
        let inverse = self.0.inverse();

        if let Some(data) = inverse{
            self.0 = data;
        }

        self
    }
}

#[cfg(test)]
mod test{
    use crate::engine::math::vector::CoOrdinate;

    #[test]
    fn scaling_with_points(){
        let point = CoOrdinate::new_point(-4.0, 6.0, 8.0);

        assert_eq!(point.scale(2.0, 3.0, 4.0), CoOrdinate::new_point(-8.0, 18.0, 32.0))
    }

    #[test]
    fn scaling_with_vectors(){
        let point = CoOrdinate::new_vector(-4.0, 6.0, 8.0);

        assert_eq!(point.scale(2.0, 3.0, 4.0), CoOrdinate::new_vector(-8.0, 18.0, 32.0))
    }

    #[test]
    // Works but with a small floating point error
    fn shrinking_using_scaling(){
        let point = CoOrdinate::new_point(-4.0, 6.0, 8.0);

        assert_eq!(point.shrink(2.0, 3.0, 4.0), CoOrdinate::new_point(-2.0, 2.0, 2.0))
    }

    #[test]
    fn reflecting_using_scaling(){
        // reflecting is scaling using negative values

        let point = CoOrdinate::new_point(-4.0, 6.0, 8.0);
        assert_eq!(point.reflection(-1.0, 1.0, 1.0), CoOrdinate::new_point(4.0, 6.0, 8.0))
    }

}

