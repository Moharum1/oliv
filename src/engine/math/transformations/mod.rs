use crate::engine::math::matrix::Matrix4X4;

pub(crate) mod translation;
pub(crate) mod scaling;
pub(crate) mod rotation;
pub(crate) mod shearing;

#[derive(PartialEq, Copy, Clone, Debug)]
// TODO: Improve the way of choosing proper operation
pub enum MatTransform{
    Translation(f32, f32, f32),
    InverseTranslation(f32, f32, f32),
    Scaling(f32, f32, f32),
    InverseScaling(f32,f32,f32),
    Rotation(f32, f32, f32),
    Sheering(f32, f32, f32, f32, f32, f32),
    Combination(Matrix4X4),
    Ideal
}