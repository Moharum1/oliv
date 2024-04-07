use std::fs::File;
use std::io::{BufWriter, Write};
use crate::engine::math::vector::CoOrdinate;
use crate::engine::view::colors::Color;

#[derive(Clone)]
pub struct
Canvas{
    pub(crate) w: usize,
    pub(crate) h: usize,
    pub(crate) map: Vec<Vec<Color>>
}

impl Canvas{
    pub fn new(w:usize, aspect_ratio : f32) -> Canvas {
        let h = (w as f32 / aspect_ratio).round() as usize;
        Canvas{
            w,
            h,
            map: vec![vec![Color::new(0f32, 0f32, 0f32); w]; h]
        }
    }

    pub fn write_pixel(&mut self, x:usize, y:usize, color: Color){
        if x < self.w && y < self.h {
            self.map[y][x] = color
        }
    }

    pub fn get_viewport_co_ordinates(&self,x: f32, y: f32 ,distance : f32) -> CoOrdinate {
        CoOrdinate::new_point(
            x * (100/self.w) as f32,
            y * (100/self.h) as f32,
            distance)
    }

    pub fn to_ppm(&self, name : &str){
        let path = "./res/{}.ppm".replace("{}", name);
        let file = File::create(path).expect("Failed to create an output");
        let mut writer = BufWriter::new(file);

        write!(writer, "P3\n{} {}\n255\n", self.w, self.h).unwrap();


        let res: Vec<_> = self.map.iter().flat_map(|row|{
            row.iter().flat_map(|color|{
                color.to_u8().clone()
            }).collect::<Vec<u8>>()
        }).collect();

        write!(writer, "{:?}", res).unwrap();
    }
}

