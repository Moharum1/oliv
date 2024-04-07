#[cfg(test)]
mod test{
    use crate::engine::view::colors::Color;

    // Note : All Basic operations works but with a floating point error

    #[test]
    fn check_color_addition(){
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 + c2;

        assert_eq!(res, Color::new(1.6, 0.7, 1.0))
    }

    #[test]
    fn check_color_sub(){
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let res = c1 - c2;

        assert_eq!(res, Color::new(0.2, 0.5, 0.5))
    }

    #[test]
    fn check_color_mul_with_scalar(){
        let c1 = Color::new(0.2, 0.3, 0.4);
        let c2 = c1 * 2;

        assert_eq!(c2, Color::new(0.4, 0.6, 0.8))
    }

    #[test]
    fn check_color_mul_with_color(){
        let c1 = Color::new(1f32, 0.2, 0.4);
        let c2 = Color::new(0.9, 1f32, 0.1);
        let res = c1 * c2;

        assert_eq!(res, Color::new(0.9, 0.2, 0.04))
    }
}