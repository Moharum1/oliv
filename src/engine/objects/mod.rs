use crate::engine::objects::sphere::Sphere;

pub mod sphere;

#[derive(PartialEq, Debug, Clone)]
pub enum Objects{
    Sphere(Sphere),
    None
}