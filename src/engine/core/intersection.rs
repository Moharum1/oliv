use crate::engine::objects::Objects;

#[derive(PartialEq, Debug, Clone)]
pub struct Intersection{
    object: Objects,
    point : f32,
}
#[derive(PartialEq, Debug)]
pub struct Intersections{
    pub(crate) data: Vec<Intersection>
}

impl Intersection{
    pub fn new(point: f32, object: Objects) -> Intersection {
        Intersection{
            object,
            point,
        }
    }
}

impl Intersections{

    pub fn hit(&self) -> Option<Intersection>{

        let mut biggest = f32::INFINITY;
        let mut intersection = Intersection { object: Objects::None, point: 0.0 };

        for intersect in self.data.iter(){
            if intersect.point < biggest && intersect.point > 0.0 {
                println!("Biggest is {}", intersect.point);
                biggest = intersect.point;
                intersection = intersect.to_owned();
            }
        }

        if biggest < f32::INFINITY {
            Some(intersection)
        } else {
            None
        }
    }
}



#[cfg(test)]
mod tests{
    use crate::engine::core::intersection::{Intersection, Intersections};
    use crate::engine::objects::Objects;
    use crate::engine::objects::sphere::Sphere;

    #[test]
    fn check_for_hit(){
        let sphere = Sphere::new();

        let intersection1 = Intersection::new(1.0, Objects::Sphere(sphere));
        let intersection2 = Intersection::new(2.0, Objects::Sphere(sphere));
        let intersection3 = Intersection::new(-1.0, Objects::Sphere(sphere));

        let intersect = Intersections{ data: vec![intersection1.clone(), intersection2, intersection3]};
        assert_eq!(intersect.hit(), Some(intersection1))
    }

}