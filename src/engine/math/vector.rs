use std::cmp::PartialEq;
use std::fmt::{Debug, Formatter};
use std::ops;
use std::ops::{Add, Mul, Sub};
use crate::engine::math::vector::CoOrdinateType::{Point, Vector};



#[derive(PartialEq,Debug,Copy, Clone)]
pub enum CoOrdinateType{
    Point = 1,
    Vector = 0
}

impl Add for CoOrdinateType {
    type Output = CoOrdinateType;

    fn add(self, rhs: Self) -> Self::Output {
        if self == Point || rhs == Point {
            return Point
        }

        return Vector
    }
}

impl Sub for CoOrdinateType {
    type Output = CoOrdinateType;

    fn sub(self, rhs: Self) -> Self::Output {
        if self == Point && rhs == Point{
            return Vector
        } else if self == Point || rhs == Point {
            return Point
        }
        return Vector
    }
}




#[derive(Copy, Clone)]
pub struct CoOrdinate{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub(crate) kind: CoOrdinateType,
}

impl CoOrdinate{
    pub fn new_vector(x:f32, y:f32, z:f32) -> CoOrdinate {
        CoOrdinate{x,y,z, kind: Vector}
    }

    pub fn new_point(x:f32, y:f32, z:f32) -> CoOrdinate {
        CoOrdinate{x, y, z, kind: Point}
    }

    pub fn negate(&mut self){
        self.x = - self.x;
        self.y = - self.y;
        self.z = - self.z;
    }

    pub fn magnitude(&self) -> f32 {
        match self.kind {
            Point => 0f32,
            Vector => (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }
    }

    pub fn normalize(&mut self) -> CoOrdinate{
        // TODO: Reduce the Number of if conditions used
        match self.kind {
            Point => {
                self.to_owned()
            },
            Vector => {
                let magnitude = self.magnitude();
                self.x =  if self.x > 0f32 {self.x / magnitude} else {self.x};
                self.y =  if self.y > 0f32 {self.y / magnitude} else {self.y};
                self.z =  if self.z > 0f32 {self.z / magnitude} else {self.z};

                self.to_owned()
            }
        }
    }

    pub fn dot(&self, rhs: &CoOrdinate) -> f32 {
        if self.kind == Vector && rhs.kind == Vector{
            return
                    self.x * rhs.x +
                    self.y * rhs.y +
                    self.z * rhs.z;
        }
        // TODO: An Error when a Point is used instead of Vector
        0f32
    }

    pub fn cross(&self, rhs: &CoOrdinate) -> CoOrdinate {
        if self.kind == Vector && rhs.kind == Vector{
            return CoOrdinate{
                x: self.y * rhs.z - self.z * rhs.y,
                y: self.z * rhs.x - self.x * rhs.z,
                z: self.x * rhs.y - self.y * rhs.x,
                kind: Vector,
            }
        }
        // TODO: An Error when a Point is used instead of Vector
        CoOrdinate{
            x: 0f32,
            y: 0f32,
            z: 0f32,
            kind: Vector,
        }
    }

    pub fn reflect(self, normal : CoOrdinate) -> CoOrdinate {
        let scalar = 2.0 * normal.dot(&self);
        self - normal * scalar
    }

}

impl Add for CoOrdinate{
    type Output = CoOrdinate;

    fn add(self, rhs: CoOrdinate) -> Self::Output {
        CoOrdinate {
            x   : self.x + rhs.x,
            y   : self.y + rhs.y,
            z   : self.z + rhs.z,
            kind: self.kind + rhs.kind,
        }
    }
}

impl Sub for CoOrdinate{
    type Output = CoOrdinate;

    fn sub(self, rhs: CoOrdinate) -> Self::Output {
        CoOrdinate {
            x   : self.x - rhs.x,
            y   : self.y - rhs.y,
            z   : self.z - rhs.z,
            kind: self.kind - rhs.kind,
        }
    }
}

impl Mul<f32> for CoOrdinate{
    type Output = CoOrdinate;

    fn mul(self, rhs: f32) -> Self::Output {
        CoOrdinate {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            kind: self.kind,
        }
    }
}

// impl Mul<CoOrdinate> for CoOrdinate{
//     type Output = CoOrdinate;
//
//     fn mul(self, rhs: CoOrdinate) -> Self::Output {
//         CoOrdinate {
//             x: self.x * rhs.x,
//             y: self.y * rhs.y,
//             z: self.z * rhs.z,
//             kind: self.kind,
//         }
//     }
// }


impl ops::Div<i32> for CoOrdinate{
    type Output = CoOrdinate;

    fn div(self, rhs: i32) -> Self::Output {
        CoOrdinate {
            x: self.x / rhs as f32,
            y: self.y / rhs as f32,
            z: self.z / rhs as f32,
            kind: self.kind,
        }
    }
}


impl Debug for CoOrdinate{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector {{ x: {}, y: {}, z: {}, point: {:?} }}", self.x, self.y, self.z, self.kind)
    }
}


impl PartialEq for CoOrdinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.kind == self.kind
    }
}