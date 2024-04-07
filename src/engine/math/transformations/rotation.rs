use crate::engine::math::matrix::Matrix4X4;
use crate::engine::math::vector::CoOrdinate;

impl CoOrdinate {

    // TODO: Implement the transpose trait for the func like Translation
    pub fn rotate_x(self, degree: f32) -> CoOrdinate {
        let x_rotation_mat = Matrix4X4 {
            rows: [
                [1.0, 0.0         , 0.0          , 0.0],
                [0.0, degree.cos(), -degree.sin(), 0.0],
                [0.0, degree.sin(), degree.cos() , 0.0],
                [0.0, 0.0         , 0.0          , 1.0]
            ]
        };

         x_rotation_mat * self
    }


    // TODO: Implement the transpose trait for the func like Translation
    pub fn inverse_rotate_x(self, degree: f32) -> CoOrdinate {
        let x_rotation_mat = Matrix4X4 {
            rows: [
                [1.0, 0.0         , 0.0          , 0.0],
                [0.0, degree.cos(), -degree.sin(), 0.0],
                [0.0, degree.sin(), degree.cos() , 0.0],
                [0.0, 0.0         , 0.0          , 1.0]
            ]
        };

        x_rotation_mat.inverse().unwrap() * self
    }


    // TODO: Implement the transpose trait for the func like Translation
    pub fn rotate_y(self, degree: f32) -> CoOrdinate {
        let y_rotation_mat = Matrix4X4 {
            rows: [
                [degree.cos(), 0.0, degree.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-degree.sin(), 0.0, degree.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        y_rotation_mat * self
    }


    // TODO: Implement the transpose trait for the func like Translation
    pub fn inverse_rotate_y(self, degree: f32) -> CoOrdinate {
        let y_rotation_mat = Matrix4X4 {
            rows: [
                [degree.cos(), 0.0, degree.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-degree.sin(), 0.0, degree.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        y_rotation_mat.inverse().unwrap() * self
    }

    // TODO: Implement the transpose trait for the func like Translation
    pub fn rotate_z(self, degree: f32) -> CoOrdinate {
        let z_rotation_mat = Matrix4X4 {
            rows: [
                [degree.cos(), -degree.sin(), 0.0, 0.0],
                [degree.sin(), degree.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        z_rotation_mat * self
    }

    // TODO: Implement the transpose trait for the func like Translation
    pub fn inverse_rotate_z(self, degree: f32) -> CoOrdinate {
        let z_rotation_mat = Matrix4X4 {
            rows: [
                [degree.cos(), -degree.sin(), 0.0, 0.0],
                [degree.sin(), degree.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ]
        };

        z_rotation_mat.inverse().unwrap() * self
    }
}

#[cfg(test)]
mod test{
    use std::f32::consts::PI;
    use crate::engine::math::vector::CoOrdinate;


    #[test]
    fn check_x_rotation(){
        let point = CoOrdinate::new_point(0.0, 1.0, 0.0);

        let res1 = point.rotate_x(PI / 4.0);
        println!("{:?}", res1);
    }

    #[test]
    fn composed_transitions(){
        let point = CoOrdinate::new_point(1.0, 0.0, 1.0)
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert_eq!(point , CoOrdinate::new_point(15.0, 0.0, 7.0))

    }
}
