#[cfg(test)]
mod test{
    use crate::engine::view::canvas::Canvas;
    use crate::engine::view::colors::Color;

    #[test]
    fn write_pixel(){
        let red = Color::new(1f32,0f32,0f32);
        let mut canvas = Canvas::new(10, (1 / 2) as f32);

        canvas.write_pixel(2, 3, red);
        assert_eq!(canvas.map[2][3],  Color::new(1f32,0f32,0f32))
    }

    #[test]
    fn check_ppm_file_converter(){
        let red = Color::new(100f32,0f32,0f32);
        let mut canvas = Canvas::new(10, (1 / 2) as f32);

        canvas.write_pixel(0, 0, red);
        canvas.to_ppm("Hola");
    }
}