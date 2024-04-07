#[cfg(test)]
mod play{
    use std::f32::consts::PI;
    use crate::engine::canvas::Canvas;
    use crate::engine::colors::Color;
    use crate::engine::math::vector::CoOrdinate;

    const WHITE: Color = Color::new(255.0, 255.0, 255.0);

    #[test]
    fn start(){

        let mut canvas = Canvas::new(50, 50);

        let center = (50/2 , 50/2);
        let radius = 12;
        canvas.write_pixel(center.0, center.1, WHITE);


        let mut point = CoOrdinate::new_point(
            (center.0 + radius) as f32,
            (center.1 + 0) as f32,
            0f32
        );

        canvas.write_pixel(point.x as usize , point.y as usize, WHITE);

        for i  in  1..12{
            point = point.rotate_x(PI/ 6.0 * i as f32);
            println!("Current Co-ordinate x: {} and y : {}", point.x, point.y);
            canvas.write_pixel(point.x as usize , point.y as usize, WHITE);
        }

        canvas.to_ppm("Clock1");

    }
}