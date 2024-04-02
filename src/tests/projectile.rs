use crate::engine::math::vector::CoOrdinate;

#[derive(Debug)]
struct Projectile{
    position: CoOrdinate, // Point
    velocity: CoOrdinate // Vector
}

struct Environment{
    gravity: CoOrdinate, //Vector
    wind   : CoOrdinate  // Vector
}

struct PlayGround{
    obj: Projectile,
    env: Environment
}

impl PlayGround {
    fn tick(&mut self) {
        let position = self.obj.position + self.obj.velocity;
        let velocity = self.obj.velocity + self.env.gravity + self.env.wind;

        self.obj = Projectile{
            position,
            velocity
        };

        println!("Current Position {:?}", &self.obj.position)
    }
}


#[cfg(test)]
mod play{
    use crate::engine::colors::Color;
    use crate::engine::canvas::{Canvas};
    use crate::engine::math::vector::CoOrdinate;
    use crate::tests::projectile::{Environment, PlayGround, Projectile};

    #[test]
    fn play_simulation(){
        let p = Projectile{
            position : CoOrdinate::new_point(0f32, 1f32, 0f32),
            velocity : CoOrdinate::new_vector(1f32, 1.8, 0f32).normalize() * 11
        };

        let v = Environment{
            gravity: CoOrdinate::new_vector(0f32, -0.1, 0f32),
            wind: CoOrdinate::new_vector(-0.01, 0f32, 0f32),
        };

        let mut ground = PlayGround{
            obj: p,
            env: v,
        };

        let mut canva = Canvas::new(950, 550);

        loop{
            ground.tick();
            canva.write_pixel(
                ground.obj.position.x as usize,
                549 - ground.obj.position.y as usize,
                Color::new(0f32, 255f32, 0f32)
            );

            if ground.obj.position.y <= 0f32 {
                break
            }
        }

        canva.to_ppm();
    }
}
