
//TODO: A Better Implementation for Matrix in order to create Matrix
//      with Variable Sizes easily


//TODO: See “An Intuitive Guide to Linear Algebra” on    BetterExplained.com

//TODO: too many redundant code here Matrix2x2 , Matrix3x3 and Matrix4x4 methods
//      Almost have the same implementation for all of them with very minor diff
//      Find a way to use OOP to reduce all the redundant code

use std::ops::Mul;

use crate::engine::math::operations::RoundToTwoDecimalPlaces;
use crate::engine::math::vector::CoOrdinate;
use crate::engine::math::vector::CoOrdinateType::Vector;

type Matrix4X1 = [f32; 4];

const IDENTITY_MAT4X4: [[f32; 4]; 4] = [
                                            [1f32 ,0.0 ,0.0 ,0.0 ],
                                            [0f32 ,1.0 ,0.0 ,0.0 ],
                                            [0f32 ,0.0 ,0.0 ,1.0 ],
                                            [0f32 ,0.0 ,0.0 ,1.0 ]
                                        ];
#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Matrix4X4{
    pub(crate) rows: [[f32; 4]; 4]
}

#[derive(PartialEq, Debug)]
pub(crate) struct Matrix3X3{
    pub(crate) rows: [[f32;3]; 3]
}

impl Matrix3X3{
    fn sub_matrix(&self, x_row: usize, x_column: usize) -> Matrix2X2 {
        let mut res = [[0.0; 2]; 2];
        let mut res_row = 0;

        for row in 0..3 {
            if row == x_row {
                continue;
            } else {
                let mut res_column = 0;
                for column in 0..3 {
                    if column == x_column {
                        continue;
                    } else {
                        res[res_row][res_column] = self.rows[row][column];
                        res_column += 1;
                    }
                }
                res_row += 1;
            }
        }

        Matrix2X2 { rows: res }
    }

    /** minor : Compute the determinant of a sub_matrix
        @x_row: the row you want to remove
        @x_column: the column you want to remove
        return: the determinant of the formed sub_matrix
    */
    pub fn minor(&self, x_row: usize, x_column: usize) -> i32 {
        self.sub_matrix(x_row, x_column).determinant()
    }

    pub fn co_factor(&self, x_row: usize, x_column: usize) -> i32{
        let determinant = self.minor(x_row, x_column);

        if (x_row + x_column) % 2 == 0 {
            determinant
        }else {
            -determinant
        }
    }

    pub fn determinant(&self) -> i32 {
            self.rows[0][0] as i32 * self.co_factor(0,0) +
            self.rows[0][1] as i32 * self.co_factor(0,1) +
            self.rows[0][2] as i32 * self.co_factor(0,2)
    }
}


#[derive(PartialEq,Debug)]
pub(crate) struct Matrix2X2{
    pub(crate) rows: [[f32; 2];2]
}

impl Matrix2X2{
    pub fn determinant(&self) -> i32 {
            (self.rows[0][0] * self.rows[1][1] -
             self.rows[0][1] * self.rows[1][0])
            as i32
    }
}


impl Matrix4X4{
    pub fn new() -> Matrix4X4 {
        Matrix4X4{
            rows: [[0.0; 4]; 4],
        }
    }

    pub fn transpose(&self) -> Matrix4X4 {
        let mut res = Matrix4X4::new();

        for row in 0..4{
            for column in 0..4{
                res.rows[column][row] =  self.rows[row][column]
            }
        }

        res
    }

    pub fn sub_matrix(&self, x_row: usize, x_column: usize) -> Matrix3X3 {
        let mut res = [[0.0; 3]; 3];
        let mut res_row = 0;

        for row in 0..4 {
            if row == x_row {
                continue;
            } else {
                let mut res_column = 0;
                for column in 0..4 {
                    if column == x_column {
                        continue;
                    } else {
                        res[res_row][res_column] = self.rows[row][column];
                        res_column += 1;
                    }
                }
                res_row += 1;
            }
        }

        Matrix3X3 { rows: res }
    }

    fn minor(&self, x_row: usize, x_column: usize) -> i32 {
        self.sub_matrix(x_row, x_column).determinant()
    }

    pub fn co_factor(&self, x_row: usize, x_column: usize) -> i32{
        let determinant = self.minor(x_row, x_column);

        if (x_row + x_column) % 2 == 0 {
            determinant
        }else {
            -determinant
        }
    }

    pub fn determinant(&self) -> i32 {
        let mut determinant = 0;

        for column in 0..4{
            determinant += self.rows[0][column] as i32 * self.co_factor(0, column)
        }

        determinant
    }

    pub fn inverse(&self) -> Option<Matrix4X4> {

        let determinant = self.determinant();

        if determinant != 0{
            let mut co_fact_matrix = Matrix4X4::new();

            for row in 0..4{
                for column in 0..4{
                    let co_factor = self.co_factor(row, column) as f32;

                    co_fact_matrix.rows[row][column] =
                        (co_factor / determinant as f32)
                            .round_to_two_decimal_places()

                }
            }

            Some(co_fact_matrix.transpose())
        }else {
            None
        }
    }
}

impl Mul<Matrix4X4> for Matrix4X4{
    type Output = Matrix4X4;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Matrix4X4::new();

        for row in 0..4{
            for column in 0..4{
                res.rows[row][column] = {
                            self.rows[row][0] * rhs.rows[0][column] +
                            self.rows[row][1] * rhs.rows[1][column] +
                            self.rows[row][2] * rhs.rows[2][column] +
                            self.rows[row][3] * rhs.rows[3][column]
                }
            }
        }

        res
    }
}

impl Mul<Matrix4X1> for Matrix4X4{
    type Output = Matrix4X1;

    fn mul(self, rhs: Matrix4X1) -> Self::Output {
        let mut res = [0.0; 4];

        for row in 0..4{
            res[row] = {
                    self.rows[row][0] * rhs[0]+
                    self.rows[row][1] * rhs[1] +
                    self.rows[row][2] * rhs[2] +
                    self.rows[row][3] * rhs[3]
            }

        }
        res
    }
}

impl Mul<CoOrdinate> for Matrix4X4 {
    type Output = CoOrdinate;

    fn mul(self, rhs: CoOrdinate) -> Self::Output {

        let type_value = (rhs.kind as i32) as f32;

        let x = {
                self.rows[0][0] * rhs.x +
                self.rows[0][1] * rhs.y +
                self.rows[0][2] * rhs.z +
                self.rows[0][3] * type_value
        };

        let y = {
                self.rows[1][0] * rhs.x +
                self.rows[1][1] * rhs.y +
                self.rows[1][2] * rhs.z +
                self.rows[1][3] * type_value
        };

        let z = {
                self.rows[2][0] * rhs.x +
                self.rows[2][1] * rhs.y +
                self.rows[2][2] * rhs.z +
                self.rows[2][3] * type_value
        };


        if rhs.kind == Vector{
            CoOrdinate::new_vector(x,y,z)
        }else {
            CoOrdinate::new_point(x,y,z)
        }
    }
}
