extern crate image;

use image::{GenericImageView};

pub struct Reader {
    path: String,
    // data: Vec<Vec<[u8; 3]>>
}

impl Reader {
    pub fn new(path: String) -> Reader {
        Reader { path }
    }

    fn near_white(pixel: (u32, u32, image::Rgba<u8>), img: &image::DynamicImage) -> bool {

        let currently_black = match pixel.2 {
            image::Rgba([0, 0, 0, 255]) => true,
            _ => false,
        };

        if !currently_black {
            return false;
        }

        let mut currently_white: bool;
        let range: i32 = 5;
        let dim = img.dimensions();
        let x_0 = pixel.0 as i32;
        let x_1 = pixel.1 as i32;

        for i in -range..range {
            for j in -range..range {

                let x = i + x_0;
                let y = j + x_1;

                if x < 0 || y < 0 { continue }
                if x > (dim.0 as i32) || y > (dim.1 as i32) { continue }
                let near_color = img.get_pixel(x as u32, y as u32);

                currently_white = match near_color {
                    image::Rgba([255, 255, 255, 255]) => true,
                    _ => false,
                };

                if currently_white {
                    return true;
                }
            }
        }
        false
    }

    pub fn read_as_line_img(&self) -> (u32, u32, Vec<Vec<[u8; 3]>>) {
        println!("reading file from path{}", self.path);
        let img = image::open(&self.path).unwrap();

        let dimensions = img.dimensions();

        let mut data = vec![];
        for _ in 0..dimensions.0 {
            data.push(vec![[0, 0, 0]; 910])
        }

        for pixel in img.pixels() {
            let near_white = Reader::near_white(pixel, &img);
            let x = pixel.0 as usize;
            let y = pixel.1 as usize;
            let color = pixel.2;
            if near_white {
                data[x][y] = [color[0], color[1], color[2]];
            } else {
                data[x][y] = [255, 255, 255];
            }
        }

        (dimensions.0, dimensions.1, data)
    }
}
