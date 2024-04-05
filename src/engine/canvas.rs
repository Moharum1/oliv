use std::fs::File;
use std::io::{BufWriter, Write};
use crate::engine::colors::Color;

#[derive(Clone)]
pub(crate) struct
Canvas{
    w: usize,
    h: usize,
    pub(crate) map: Vec<Vec<Color>>
}

impl Canvas{
    pub(crate) fn new(w:usize, h:usize) -> Canvas {
        Canvas{
            w,
            h,
            map: vec![vec![Color::new(0f32, 0f32, 0f32); w]; h]
        }
    }

    pub(crate) fn write_pixel(&mut self, x:usize, y:usize, color: Color){
        if x < self.w && y < self.h {
            self.map[y][x] = color
        }
    }

    pub(crate) fn to_ppm(&self, name : &str){
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

