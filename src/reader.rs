use std::fs;
use std::io::Read;
use std::process::exit;

use svg2polylines::{self, Polyline};

pub struct Reader {}

impl Reader {
    pub fn read(path: String) -> Vec<[u32; 2]> {
        // Load file
        let mut file = fs::File::open(path).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();

        // Parse data
        let polylines: Vec<Polyline> = svg2polylines::parse(&s).unwrap_or_else(|e| {
            println!("Error: {}", e);
            exit(2);
        });

        let mut min_x = 10000.;
        let mut min_y = 10000.;
        for line in &polylines {
            for pair in line {
                if min_x > pair.x {
                    min_x = pair.x;
                }
                if min_y > pair.y {
                    min_y = pair.y;
                }
            }
        }

        let mut numbers: Vec<[u32; 2]> = vec![];
        for line in polylines {
            for pair in line {
                numbers.push([
                    ((pair.x - min_x) * 0.1) as u32 + 260,
                    ((pair.y - min_y) * 0.1) as u32 + 200,
                ])
            }
        }
        numbers
    }
}
