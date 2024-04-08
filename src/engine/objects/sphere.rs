use std::ops::Mul;
use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use crate::engine::core::intersection::{Intersection, Intersections};
use crate::engine::core::ray::Ray;
use crate::engine::math::vector::CoOrdinate;
use crate::engine::math::matrix::{IDENTITY_MAT4X4, Matrix4X4};
use crate::engine::math::transformations::MatTransform;
use crate::engine::math::transformations::MatTransform::{Combination, Ideal, InverseScaling, InverseTranslation, Scaling, Translation};
use crate::engine::math::transformations::scaling::Scale;
use crate::engine::objects::Objects;
use crate::engine::math::transformations::translation::Translate;

static UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere{
    id         : usize,
    pos        : CoOrdinate,
    radius     : f32,
    transform  : MatTransform
}

impl Sphere{
    pub fn new() -> Sphere {
        let id = UNIQUE_ID.fetch_add(1, Ordering::Relaxed);
        Sphere{
            id,
            pos: CoOrdinate::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Ideal
        }
    }

    pub fn intersect(&self, ray: Ray) -> Option<Intersections> {

        let transformed_ray = ray.transform(self.transform);

        let sphere_to_ray = transformed_ray.origin - CoOrdinate::new_point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&transformed_ray.direction);
        let b = ray.direction.dot(&sphere_to_ray) * 2.0;
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;

        let discriminate = b.powi(2) - 4.0 * a * c; // Correcting the calculation of the discriminant

        if discriminate < 0.0 {
            None
        } else {
            let t1 = (-b - discriminate.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminate.sqrt()) / (2.0 * a);

            Some(
                Intersections{
                    data : vec![
                    Intersection::new(t1, Objects::Sphere(self.clone())),
                    Intersection::new(t2, Objects::Sphere(self.clone())),]
                }
            )
        }
    }

    pub fn set_transform(&mut self, transform : MatTransform){

        // Function will match the transform and apply it to the sphere transform trait
        match transform {
            Translation(x, y, z) => {
                self.transform = Translation(x,y,z)
            }
            Scaling(x, y, z) => {
                self.transform = Scaling(x,y,z)
            }

            InverseTranslation(x,y,z) => {
                self.transform = InverseTranslation(x,y,z)
            }

            InverseScaling(x,y,z) => {
                self.transform = InverseScaling(x,y,z)
            }

            Combination(Mat) => {
                self.transform = Combination(Mat)
            }

            _ => {
            }
        }
    }

    pub fn normal_at(&self, world_point : CoOrdinate) -> CoOrdinate {

        let object_pos;
        let sphere_transpose_coord;

        // TODO : SOLVE the issue in normal calculation
        match self.transform {
            Translation(x, y, z) => {
                object_pos = world_point.inverse_translate(x, y, z);
                sphere_transpose_coord = Translate::new(x,y,z).0.inverse().unwrap().transpose();
            }

            Scaling(x, y, z) => {
                object_pos =  world_point.shrink(x, y, z);
                sphere_transpose_coord = Scale::new(x,y,z).0.inverse().unwrap().transpose();
            }

            Combination(Mat) => {
                println!("{:?}", Mat);
                object_pos = Mat.inverse().unwrap_or(IDENTITY_MAT4X4) * world_point;
                sphere_transpose_coord = Mat.inverse().unwrap_or(IDENTITY_MAT4X4).transpose();
            }

            _ => {
                return world_point - self.pos;
            }
        }

        let object_normal = object_pos - self.pos;
        let mut world_normal = sphere_transpose_coord * object_normal;

        return world_normal.normalize();
    }
}

#[cfg(test)]
mod test{
    use std::f32::consts::PI;
    use crate::engine::core::intersection::{Intersection, Intersections};
    use crate::engine::core::ray::Ray;
    use crate::engine::math::transformations::MatTransform;
    use crate::engine::math::transformations::MatTransform::Rotation;
    use crate::engine::math::transformations::rotation::Rotate;
    use crate::engine::math::transformations::scaling::Scale;
    use crate::engine::math::transformations::translation::Translate;
    use crate::engine::math::vector::CoOrdinate;
    use crate::engine::objects::Objects;
    use crate::engine::objects::sphere::Sphere;

    #[test]
    fn check_similarity(){
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();

        assert_ne!(sphere1, sphere2);
    }

    #[test]
    fn check_sphere_intersection(){
        let ray  = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let ray2 = Ray::new((0.0, 1.0, -5.0), (0.0, 0.0, 1.0));
        let ray3 = Ray::new((0.0, 2.0, -5.0), (0.0, 0.0, 1.0));
        let ray4 = Ray::new((0.0, 0.0, 0.0), (0.0, 0.0, 1.0));
        let ray5 = Ray::new((0.0, 0.0, 5.0), (0.0, 0.0, 1.0));

        let sphere = Sphere::new();

        let res = sphere.intersect(ray).unwrap();
        let res2 = sphere.intersect(ray2).unwrap();
        let res3 = sphere.intersect(ray3);
        let res4 = sphere.intersect(ray4).unwrap();
        let res5 = sphere.intersect(ray5).unwrap();

        let intersect1 = Intersections{
            data: vec![Intersection::new(4.0, Objects::Sphere(sphere)), Intersection::new(6.0, Objects::Sphere(sphere)),]
        };
        let intersect2 = Intersections{
            data: vec![Intersection::new(5.0, Objects::Sphere(sphere)), Intersection::new(5.0, Objects::Sphere(sphere)),]
        };
        let intersect3 = Intersections{
            data: vec![Intersection::new(-1.0, Objects::Sphere(sphere)), Intersection::new(1.0, Objects::Sphere(sphere)),]
        };
        let intersect4 = Intersections{
            data: vec![Intersection::new(-6.0, Objects::Sphere(sphere)), Intersection::new(-4.0, Objects::Sphere(sphere)),]
        };


        assert_eq!(res, intersect1);
        assert_eq!(res2,intersect2);
        assert_eq!(res3, None);
        assert_eq!(res4,intersect3);
        assert_eq!(res5,intersect4);
    }

    #[test]
    fn check_intersection_with_transformation(){
        let ray = Ray::new((0.0, 0.0, -5.0), (0.0, 0.0, 1.0));
        let mut sphere: Sphere = Sphere::new();

        sphere.set_transform(MatTransform::InverseTranslation(5.0, 0.0, 0.0));
        println!("{:?}", sphere.intersect(ray))
    }

    #[test]
    fn check_normal(){
        let s = Sphere::new();
        let point1 = CoOrdinate::new_point(0.0, 1.0, 0.0);
        assert_eq!(s.normal_at(point1), CoOrdinate::new_vector(0.0, 1.0, 0.0))
    }

    #[test]
    fn compute_normal_for_translated_spheres(){
        let mut s = Sphere::new();

        s.set_transform(MatTransform::Translation(0.0, 1.0, 0.0));

        let point1 = CoOrdinate::new_point(0.0, 1.7071,  -0.70711);
        assert_eq!(s.normal_at(point1), CoOrdinate::new_vector(0.0, 0.7071018, -0.70711))
    }

    #[test]
    fn compute_normal_for_translated_spheres2(){
        let mut s = Sphere::new();
        s.set_transform(MatTransform::Combination(Scale::new(1.0, 0.5, 1.0).0 *  Rotate::new().rotate_z(PI / 5.0)));

        let point1 = CoOrdinate::new_point(0.0, 0.7071,  -0.7071);
        assert_eq!(s.normal_at(point1), CoOrdinate::new_vector(0.0, 0.97014, -0.24254))
    }
}