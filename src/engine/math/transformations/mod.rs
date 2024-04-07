use crate::engine::math::matrix::Matrix4X4;

mod translation;
mod scaling;
mod rotation;
mod shearing;

#[derive(PartialEq, Copy, Clone, Debug)]
// TODO: Improve the way of choosing proper operation
pub enum MatTransform{
    Translation(f32, f32, f32),
    InverseTranslation(f32, f32, f32),
    Scaling(f32, f32, f32),
    InverseScaling(f32,f32,f32),
    Rotation(f32, f32, f32),
    Sheering(f32, f32, f32, f32, f32, f32),
    Ideal
}