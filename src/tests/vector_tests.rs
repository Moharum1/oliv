#[cfg(test)]
mod test{
    use crate::engine::math::vector::CoOrdinate;

    #[test]
    fn check_point_or_vector(){
        let vec1 = CoOrdinate::new_vector(1.0, 2.0, 3.0);
        let point1 = CoOrdinate::new_point(1.0, 2.0, 3.0);

        assert_ne!(point1, vec1)
    }

    #[test]
    fn add_point_and_vector(){
        let vec1 = CoOrdinate::new_vector(1.0, 2.0, 3.0);
        let point1 = CoOrdinate::new_point(1.0, 2.0, 3.0);

        let res = vec1 + point1;
        assert_eq!(res, CoOrdinate::new_point(2.0, 4.0, 6.0))
    }

    #[test]
    fn sub_point_and_vector(){
        let mut point2 = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let point1 = CoOrdinate::new_point(5.0, 6.0, 7.0);
        let mut vec1 = CoOrdinate::new_vector(5.0, 6.0, 7.0);

        let res1 = point2 - point1;

        point2 = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let res2 = point2 - vec1;

        vec1 = CoOrdinate::new_vector(3.0, 2.0 ,1.0);
        let vec2 = CoOrdinate::new_vector(5.0, 6.0, 7.0);
        let res3 = vec1 - vec2;

        assert_eq!(res1, CoOrdinate::new_vector(-2.0, -4.0, -6.0));
        assert_eq!(res2, CoOrdinate::new_point(-2.0, -4.0, -6.0));
        assert_eq!(res3, CoOrdinate::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn check_mul_and_div(){
        let mut point = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let res1 = point * 2;

        point = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let res2 = point / 2;
        assert_eq!(res1 , CoOrdinate::new_point(6.0, 4.0, 2.0));
        assert_eq!(res2 , CoOrdinate::new_point(1.5, 1.0, 0.5));
    }

    #[test]
    fn check_magnitude(){
        let vec = CoOrdinate::new_vector(3.0, 2.0, 1.0);
        assert_eq!(vec.magnitude(), 14f32.sqrt());
    }

    #[test]
    fn check_negation(){
        let mut vec = CoOrdinate::new_vector(3.0, 2.0, 1.0);
        vec.negate();
        assert_eq!(vec, CoOrdinate::new_vector(-3.0, -2.0, -1.0))
    }
}