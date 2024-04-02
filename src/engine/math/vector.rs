use std::cmp::PartialEq;
use std::ops;
use std::ops::{Add, Sub};
use crate::engine::math::vector::CoOrdinateType::{Point, Vector};



#[derive(PartialEq,Debug,Copy, Clone)]
enum CoOrdinateType{
    Point,
    Vector
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




#[derive(PartialEq,Debug,Copy, Clone)]
pub struct CoOrdinate{
    pub(crate) x: f32,
    pub y: f32,
    z: f32,
    kind: CoOrdinateType
}

impl CoOrdinate{
    pub fn new_vector(x:f32, y:f32, z:f32) -> CoOrdinate {
        CoOrdinate{x,y,z, kind: Vector}
    }

    pub fn new_point(x:f32, y:f32, z:f32) -> CoOrdinate {
        CoOrdinate{x, y, z, kind: Point }
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

impl ops::Mul<i32> for CoOrdinate{
    type Output = CoOrdinate;

    fn mul(self, rhs: i32) -> Self::Output {
        CoOrdinate {
            x: self.x * rhs as f32,
            y: self.y * rhs as f32,
            z: self.z * rhs as f32,
            kind: self.kind,
        }
    }
}

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


#[cfg(test)]
mod test{
    use crate::engine::math::vector::CoOrdinate;

    #[test]
    fn check_point_or_vector(){
        let vec1 = CoOrdinate::new_vector(1.0, 2.0, 3.0);
        let point1 = CoOrdinate::new_point(1.0, 2.0, 3.0);

        assert_ne!(point1, vec1)
    }

    #[test]
    fn add_point_and_vector(){
        let vec1 = CoOrdinate::new_vector(1.0, 2.0, 3.0);
        let point1 = CoOrdinate::new_point(1.0, 2.0, 3.0);

        let res = vec1 + point1;
        assert_eq!(res, CoOrdinate::new_point(2.0, 4.0, 6.0))
    }

    #[test]
    fn sub_point_and_vector(){
        let mut point2 = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let point1 = CoOrdinate::new_point(5.0, 6.0, 7.0);
        let mut vec1 = CoOrdinate::new_vector(5.0, 6.0, 7.0);

        let res1 = point2 - point1;

        point2 = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let res2 = point2 - vec1;

        vec1 = CoOrdinate::new_vector(3.0, 2.0 ,1.0);
        let vec2 = CoOrdinate::new_vector(5.0, 6.0, 7.0);
        let res3 = vec1 - vec2;

        assert_eq!(res1, CoOrdinate::new_vector(-2.0, -4.0, -6.0));
        assert_eq!(res2, CoOrdinate::new_point(-2.0, -4.0, -6.0));
        assert_eq!(res3, CoOrdinate::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn check_mul_and_div(){
        let mut point = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let res1 = point * 2;

        point = CoOrdinate::new_point(3.0, 2.0, 1.0);
        let res2 = point / 2;
        assert_eq!(res1 , CoOrdinate::new_point(6.0, 4.0, 2.0));
        assert_eq!(res2 , CoOrdinate::new_point(1.5, 1.0, 0.5));
    }

    #[test]
    fn check_magnitude(){
        let vec = CoOrdinate::new_vector(3.0, 2.0, 1.0);
        assert_eq!(vec.magnitude(), 14f32.sqrt());
    }

    #[test]
    fn check_negation(){
        let mut vec = CoOrdinate::new_vector(3.0, 2.0, 1.0);
        vec.negate();
        assert_eq!(vec, CoOrdinate::new_vector(-3.0, -2.0, -1.0))
    }
}