use std::sync::atomic::{AtomicUsize, Ordering};
use crate::engine::math::vector::CoOrdinate;
use crate::engine::ray::Ray;

static UNIQUE_ID: AtomicUsize = AtomicUsize::new(0);


#[derive(PartialEq, Debug, Copy, Clone)]
struct Sphere{
    id: usize,
    pos: CoOrdinate,
    radius: f32
}

impl Sphere{
    pub fn new() -> Sphere {
        let id = UNIQUE_ID.fetch_add(1, Ordering::Relaxed);
        println!("New Sphere created with ID: {}", id);
        Sphere{
            id,
            pos: CoOrdinate::new_point(0.0, 0.0, 0.0),
            radius: 1.0
        }
    }

    pub fn intersect(ray : Ray){
        
    }
}

#[cfg(test)]
mod test{
    use crate::engine::objects::sphere::Sphere;

    #[test]
    fn check_similarity(){
        let sphere1 = Sphere::new();
        let sphere2 = Sphere::new();
        let sphere3 = Sphere::new();

        assert_ne!(sphere1.id, sphere1.id)
    }
}