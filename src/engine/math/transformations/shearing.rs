use crate::engine::math::matrix::Matrix4X4;
use crate::engine::math::vector::CoOrdinate;

struct Shearing(Matrix4X4);

impl CoOrdinate{

    pub fn shearing(self, xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> CoOrdinate {
        let shearing_mat = Matrix4X4{
            rows: [
                [1.0, xy , xz , 0.0],
                [yx , 1.0, yz , 0.0],
                [zx , zy , 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ],
        };

        shearing_mat * self
    }
}

pub struct Sheer(pub Matrix4X4);

impl Sheer{
    pub fn new(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix4X4 {
        Matrix4X4{
            rows: [
                [1.0, xy , xz , 0.0],
                [yx , 1.0, yz , 0.0],
                [zx , zy , 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ],
        }
    }
}

mod test{
    use crate::engine::math::vector::CoOrdinate;

    #[test]
    fn check_cheering(){
        let point = CoOrdinate::new_point(2.0, 3.0, 4.0);


        assert_eq!(point.shearing(1.0, 0.0, 0.0, 0.0 ,0.0, 0.0), CoOrdinate::new_point(5.0, 3.0, 4.0));
        assert_eq!(point.shearing(0.0, 0.0, 1.0, 0.0 ,0.0, 0.0), CoOrdinate::new_point(2.0, 5.0, 4.0));
        assert_eq!(point.shearing(0.0, 0.0, 0.0, 1.0 ,0.0, 0.0), CoOrdinate::new_point(2.0, 7.0, 4.0));
    }
}