use std::ops::Mul;
use crate::engine::math::matrix::Matrix4X4;
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
    pub fn reflect(self, x:f32, y:f32, z:f32) -> CoOrdinate{
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
        assert_eq!(point.reflect(-1.0, 1.0, 1.0), CoOrdinate::new_point(4.0, 6.0, 8.0))
    }

}

