use crate::engine::math::vector::CoOrdinate;

pub(crate) struct Ray{
    origin: CoOrdinate, // Point
    direction : CoOrdinate // Vector
}

impl Ray{
    pub fn new(origin: (f32, f32, f32), dir: (f32, f32, f32)) -> Ray {
        Ray{
            origin: CoOrdinate::new_point(origin.0, origin.1, origin.2),
            direction: CoOrdinate::new_vector(dir.0, dir.1, dir.2)
        }
    }

    pub fn position(&self, t: f32) -> CoOrdinate {
        CoOrdinate::new_point(
            self.origin.x + (self.direction.x * t),
            self.origin.y + (self.direction.y * t),
            self.origin.z + (self.direction.z * t),
        )
    }
}


#[cfg(test)]
mod tests{
    use crate::engine::math::vector::CoOrdinate;
    use crate::engine::ray::Ray;

    #[test]
    fn creating_a_ray(){
        let origin = CoOrdinate::new_point(1.0, 2.0, 3.0);
        let direction = CoOrdinate::new_vector(4.0, 5.0, 6.0);

    }

    #[test]
    fn compute_distance(){
        let ray = Ray::new((2.0, 3.0, 4.0), (1.0, 0.0, 0.0));

        let p1 = ray.position(0.0);
        let p2 = ray.position(1.0);
        let p3 = ray.position(-1.0);
        let p4 = ray.position(2.5);

        assert_eq!(p1, CoOrdinate::new_point(2.0, 3.0, 4.0));
        assert_eq!(p2, CoOrdinate::new_point(3.0, 3.0, 4.0));
        assert_eq!(p3, CoOrdinate::new_point(1.0, 3.0, 4.0));
        assert_eq!(p4, CoOrdinate::new_point(4.5, 3.0, 4.0));

    }
}