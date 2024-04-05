#[cfg(test)]
mod test{
    use crate::engine::math::matrix::{Matrix2X2, Matrix3X3, Matrix4X4};

    #[test]
    fn check_matrix_impl(){
        let mut mat = Matrix4X4::new();
        let mut mat2 = Matrix4X4::new();

        for i in 0..4{
            for j in 0..4{
                mat.rows[i][j] = (i + j) as f32;
            }
        }

        for i in 0..4{
            for j in 0..4{
                mat2.rows[i][j] = (i + j + 2) as f32;
            }
        }

        assert_ne!(mat, mat2)
    }

    #[test]
    fn check_mul(){
        let mut mat1 = Matrix4X4::new();
        let mut mat2 = Matrix4X4::new();
        let mut res = Matrix4X4::new();

        mat1.rows = [[1f32,2f32,3f32,4f32],[5f32,6f32,7f32,8f32],[9f32,8f32,7f32,6f32],[5f32,4f32,3f32,2f32]];
        mat2.rows = [[-2f32,1f32,2f32,3f32], [3f32,2f32,1f32,-1f32], [4f32,3f32,6f32,5f32], [1f32,2f32,7f32,8f32]];
        res.rows = [[20f32,22f32,50f32,48f32], [44f32,54f32,114f32,108f32], [40f32,58f32,110f32,102f32], [16f32,26f32,46f32,42f32]];

        let my_res = mat1 * mat2;
        assert_eq!(my_res, res);
    }

    #[test]
    fn check_mul_with_1d_matrix(){
        let mut mat1 = Matrix4X4::new();

        mat1.rows = [[1f32,2f32,3f32,4f32],[2f32,4f32,4f32,2f32],[8f32,6f32,4f32,1f32],[0f32,0f32,0f32,1f32]];
        let mat2 = [1f32,2f32,3f32,1f32];
        let res = [18f32, 24f32, 33f32, 1f32];

        let my_res = mat1 * mat2;
        assert_eq!(my_res, res);
    }

    #[test]
    fn check_transpose(){
        let mut mat1 = Matrix4X4::new();
        mat1.rows = [[0f32,9f32,3f32,0f32],[9f32,8f32,0f32,8f32],[1f32,8f32,5f32,3f32],[0f32,0f32,5f32,8f32]];
        let result = mat1.transpose();

        let res = [[0f32,9f32,1f32,0f32],[9f32,8f32,8f32,0f32],[3f32,0f32,5f32,5f32],[0f32,8f32,3f32,8f32]];

        assert_eq!(result.rows, res)
    }

    #[test]
    fn check_determinant(){
        let mat = Matrix2X2{
            rows : [[1.0, 5.0], [-3.0, 2.0]]
        };

        assert_eq!(17, mat.determinant())
    }

    #[test]
    fn check_sub_matrix(){
        let mat = Matrix4X4{
            rows : [[1.0, 5.0, 0.0, 0.0], [-3.0, 2.0, 7.0, 0.0], [0.0, 6.0, -3.0, 0.0], [0f32, 0f32, 0f32, 0f32]]
        };

        let res = [[-3.0, 2.0, 0.0], [0.0, 6.0, 0.0], [0f32, 0f32, 0f32]];

        assert_eq!(mat.sub_matrix(0,2), Matrix3X3{rows: res})
    }

    #[test]
    fn check_minor(){
        let mat = Matrix3X3{
            rows : [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]
        };

        assert_eq!(mat.minor(1,0), 25);
        assert_eq!(mat.minor(0,0), -12)
    }

    #[test]
    fn check_3x3_matrix_determinant(){
        let mat = Matrix3X3{
            rows : [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]
        };

        assert_eq!(mat.co_factor(0,0), 56);
        assert_eq!(mat.co_factor(0,1), 12);
        assert_eq!(mat.co_factor(0,2), -46);

        assert_eq!(mat.determinant(), -196)
    }

    #[test]
    fn check_4x4_matrix_determinant(){
        let mat = Matrix4X4{
            rows : [[-2.0, -8.0, 3.0, 5.0],
                [-3.0, 1.0, 7.0, 3.0],
                [1.0, 2.0, -9.0, 6.0],
                [-6.0, 7.0, 7.0, -9.0]]
        };

        assert_eq!(mat.co_factor(0,0), 690);
        assert_eq!(mat.co_factor(0,1), 447);
        assert_eq!(mat.co_factor(0,2), 210);
        assert_eq!(mat.co_factor(0,3), 51);

        assert_eq!(mat.determinant(), -4071)
    }

    #[test]
    fn check_inversion(){
        let mat = Matrix4X4{
            rows : [[8.0, -5.0, 9.0, 2.0],
                [7.0, 5.0, 6.0, 1.0],
                [-6.0, 0.0, 9.0, 6.0],
                [-3.0, 0.0, -9.0, -4.0]]
        };

        let res = [
            [ -0.15385 , -0.15385 , -0.28205 , -0.53846],
            [-0.07692 , 0.12308 , 0.02564 , 0.03077],
            [0.35897 , 0.35897 , 0.43590 , 0.92308],
            [-0.69231 , -0.69231 , -0.76923 , -1.92308]
        ];

        let mat2 = Matrix4X4{
            rows : [[9.0, 3.0, 0.0, 9.0],
                [-5.0, -2.0, -6.0, -3.0],
                [-4.0, 9.0, 6.0, 4.0],
                [-7.0, 6.0, 6.0, 2.0]]
        };

        let res2 = [
            [ -0.04074 , -0.07778 , 0.14444 , -0.22222],
            [ -0.07778 , 0.03333 , 0.36667 , -0.33333],
            [ -0.02901 , -0.14630 , -0.10926 , 0.12963],
            [ 0.17778 , 0.06667 , -0.26667 , 0.33333]
        ];

        let mat_a = Matrix4X4{
            rows : [[3.0, -9.0, 7.0, 3.0],
                [3.0, -8.0, 2.0, -9.0],
                [-4.0, 4.0, 4.0, 1.0],
                [-6.0, 5.0, -1.0, 1.0]]
        };

        let mat_b = Matrix4X4{
            rows : [[8.0, 2.0, 2.0, 2.0],
                [3.0, -1.0, 7.0, 0.0],
                [7.0, 0.0, 5.0, 4.0],
                [6.0, -2.0, 0.0, 5.0]]
        };

        // let mat_c = mat_a.clone() * mat_b.clone();

        assert_eq!(mat.inverse().unwrap().rows, res);
        assert_eq!(mat2.inverse().unwrap().rows, res2);
    }

}