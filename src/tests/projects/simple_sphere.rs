#[cfg(test)]
mod render {
    use crate::engine::core::ray::Ray;
    use crate::engine::math::transformations::MatTransform;
    use crate::engine::math::vector::CoOrdinate;
    use crate::engine::objects::sphere::Sphere;
    use crate::engine::view::canvas::Canvas;
    use crate::engine::view::colors::Color;
    #[test]
    fn render_sphere(){
        let ray_origin = CoOrdinate::new_point(-1.0, -1.0, -7.0);
        let world_z = 10.0; // co-ordinate of the wall in z
        let wall_size = 10.0;
        let canvas_pixels = 300;
        let pixel_size = wall_size / canvas_pixels as f32;
        let half = wall_size / 2.0;

        let mut canvas = Canvas::new(canvas_pixels, 1.0);
        let red = Color::new(255.0, 0.0, 0.0);
        let mut shape = Sphere::new(); // Making the sphere immutable as we're not changing it in the loop

        // shape.set_transform(MatTransform::InverseTranslation(1.0, 1.0, 4.0));
        for y in 0..canvas_pixels {
            let world_y = half - pixel_size * y as f32;

            for x in 0..canvas_pixels {
                let world_x = -half + pixel_size * x as f32;
                let position = CoOrdinate::new_point(world_x, world_y, world_z);
                let ray_direction = (position - ray_origin).normalize();
                let ray = Ray::new((ray_origin.x , ray_origin.y, ray_origin.z), (ray_direction.x, ray_direction.y, ray_direction.z));
                let intersections = shape.intersect(ray);

                if intersections.is_some() { // Check if there are any intersections
                    canvas.write_pixel(x, y, red);
                }
            }
        }

        canvas.to_ppm("Sphere");
    }
}