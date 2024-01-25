use image::io::Reader as ImageReader;
use crate::point::Point;

/// 3x3 = 9 bits, represented as a u16
#[derive(Debug, Clone, Default)]
pub struct Tile3x3([[bool; 3]; 3]);

pub const NW_PT: Point = Point {x: 0, y: 0};
pub const N_PT: Point = Point {x: 1, y: 0};
pub const NE_PT: Point = Point {x: 2, y: 0};
pub const W_PT: Point = Point {x: 0, y: 1};
pub const C_PT: Point = Point {x: 1, y: 1};
pub const E_PT: Point = Point {x: 2, y: 1};
pub const SW_PT: Point = Point {x: 0, y: 2};
pub const S_PT: Point = Point {x: 1, y: 2};
pub const SE_PT: Point = Point {x: 2, y: 2};

impl Tile3x3 {
    pub fn get(&self, pt: &Point) -> bool {
        self.0[pt.y as usize][pt.x as usize]
    }

    pub fn set(&mut self, pt: &Point, value: bool) {
        self.0[pt.y as usize][pt.x as usize] = value
    }
}

/// generates a minimal 3x3 tileset based on image data
pub fn minimal_3x3_tile_set() -> Vec<Tile3x3> {
    let rows = 12;
    let cols = 4;

    let mut tiles = Vec::with_capacity(rows * cols);

    let tile_size_px = 64;
    let chunk_size_px = tile_size_px/3;
    let chunk_center_px = chunk_size_px/2;

    let img = ImageReader::open("../3x3-minimal.png").unwrap();
    let decoded = img.decode().unwrap();
    let rgba = decoded.as_rgba8().unwrap();

    //start with a fixed offset so when we move across the tiles we're sampling the center of the 'pixel'
    let mut sample_y = chunk_center_px;
    for _ in 0..cols {
        let mut sample_x = chunk_center_px;
        for _ in 0..rows {
            let mut tile : Tile3x3 = Tile3x3([[false; 3]; 3]);

            // read tile by sampling the center of each chunk
            let mut i = 0;
            for chunk_y in 0..3 {
                for chunk_x in 0..3 {
                    let sample = rgba.get_pixel(
                        sample_x + chunk_x * chunk_size_px,
                        sample_y + chunk_y * chunk_size_px
                    );

                    // if the chunk has a pixel with white, it's not set
                    let is_white = sample.0[0] == 255 && sample.0[1] == 255 && sample.0[2] == 255;
                    tile.set(&Point{x: chunk_x as i32, y: chunk_y as i32}, !is_white);

                    i += 1;
                }
            }

            tiles.push(tile);

            // move to next x
            sample_x += tile_size_px;
        }

        // move to next y
        sample_y += tile_size_px;
    }


    return tiles;
}
