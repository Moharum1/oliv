use std::sync::atomic::Ordering;
use std::sync::atomic::AtomicUsize;
use crate::engine::core::intersection::{Intersection, Intersections};
use crate::engine::core::ray::Ray;
use crate::engine::math::vector::CoOrdinate;
use crate::engine::math::matrix::{IDENTITY_MAT4X4, Matrix4X4};
use crate::engine::math::transformations::MatTransform;
use crate::engine::math::transformations::MatTransform::{Ideal, InverseScaling, InverseTranslation, Scaling, Translation};
use crate::engine::objects::Objects;

static UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Sphere{
    id         : usize,
    pos        : CoOrdinate,
    pos_matrix : Matrix4X4,
    radius     : f32,
    transform  : MatTransform
}

impl Sphere{
    pub fn new() -> Sphere {
        let id = UNIQUE_ID.fetch_add(1, Ordering::Relaxed);
        Sphere{
            id,
            pos: CoOrdinate::new_point(0.0, 0.0, 0.0),
            pos_matrix: IDENTITY_MAT4X4,
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

            _ => {
            }
        }
    }

    pub fn normal_at(&self, point : CoOrdinate) -> CoOrdinate{

        let object_pos;
        let sphere_transpose_coord;

        match self.transform {
            Translation(x, y, z) => {
                object_pos = point.inverse_translate(x, y, z, None);
                sphere_transpose_coord = self.pos_matrix.inverse().expect("Can't be inverse").transpose();
            }
            Scaling(x, y, z) => {
                object_pos = point.inverse_translate(x, y, z, None);
                sphere_transpose_coord = self.pos_matrix.inverse().expect("Can't be inverse").transpose();
            }
            _ => {
                object_pos = point;
                sphere_transpose_coord = self.pos_matrix.inverse().expect("Can't be inverse").transpose();
            }
        };

        let object_normal = object_pos - self.pos;
        let mut world_normal = sphere_transpose_coord * object_normal;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod test{
    use crate::engine::core::intersection::{Intersection, Intersections};
    use crate::engine::core::ray::Ray;
    use crate::engine::math::transformations::MatTransform;
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

        let point1 = CoOrdinate::new_point(0.0, 1.70711,  -0.70711);
        assert_eq!(s.normal_at(point1), CoOrdinate::new_vector(0.0, 0.70711, -0.70711))
    }
}