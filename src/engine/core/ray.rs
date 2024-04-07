use std::ops::Mul;
use crate::engine::math::transformations::{MatTransform,};
use crate::engine::math::vector::CoOrdinate;

#[derive(PartialEq, Debug)]
pub struct Ray{
    pub(crate) origin: CoOrdinate, // Point
    pub(crate) direction : CoOrdinate // Vector
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

    pub fn transform(&self, translation: MatTransform) -> Ray {

        match translation {
            MatTransform::Translation(x, y, z) => {
                Ray{
                    origin:    self.origin.translate(x, y, z),
                    direction: self.direction.translate(x, y, z),
                }
            }
            MatTransform::Scaling(x, y, z) => {
                Ray{
                    origin:    self.origin.scale(x, y, z),
                    direction: self.direction.scale(x, y, z),
                }
            }

            MatTransform::InverseTranslation(x, y, z) => {
                Ray{
                    origin:    self.origin.inverse_translate(x, y, z, None),
                    direction: self.direction.inverse_translate(x, y, z, None),
                }
            }

            MatTransform::InverseScaling(x , y, z) => {
                Ray{
                    origin:    self.origin.shrink(x, y, z),
                    direction: self.direction.shrink(x, y, z),
                }
            }

            _ => {
                Ray{
                    origin:    self.origin,
                    direction: self.direction,
                }
            }
        }


    }
}


#[cfg(test)]
mod tests{
    use png::Transformations;
    use crate::engine::core::ray::Ray;
    use crate::engine::math::transformations::MatTransform;
    use crate::engine::math::vector::CoOrdinate;

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

    #[test]
    fn check_ray_transformations(){
        let ray = Ray::new((1.0, 2.0, 3.0), (0.0, 1.0, 0.0));

        let res1 = ray.transform(MatTransform::Translation(3.0, 4.0, 5.0));
        let res2 = ray.transform(MatTransform::Scaling(2.0, 3.0, 4.0));

        assert_eq!(res1, Ray::new((4.0, 6.0, 8.0), (0.0, 1.0, 0.0)));
        assert_eq!(res2, Ray::new((2.0, 6.0, 12.0), (0.0, 3.0, 0.0)))
    }
}