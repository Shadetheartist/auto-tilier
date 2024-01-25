use image::io::Reader as ImageReader;

/// 3x3 = 9 bits, represented as a u16
#[derive(Debug, Clone, Default)]
pub struct Tile3x3(pub [bool; 9]);

pub const NW_IDX: usize = 0;
pub const N_IDX: usize = 1;
pub const NE_IDX: usize = 2;
pub const W_IDX: usize = 3;
pub const C_IDX: usize = 4;
pub const E_IDX: usize = 5;
pub const SW_IDX: usize = 6;
pub const S_IDX: usize = 7;
pub const SE_IDX: usize = 8;

impl Tile3x3 {
    #[inline]
    pub fn idx(x: u8, y: u8) -> usize {
        (y * 3 + x) as usize
    }

    #[inline]
    pub fn set_pt(&mut self, x: u8, y: u8, value: bool) {
        self.set(Self::idx(x, y), value)
    }

    #[inline]
    pub fn get_pt(&self, x: u8, y: u8) -> bool {
        self.get(Self::idx(x, y))
    }

    #[inline]
    pub fn get(&self, idx: usize) -> bool {
        self.0[idx]
    }

    #[inline]
    pub fn set(&mut self, idx: usize, value: bool) {
        self.0[idx] = value
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
            let mut tile : Tile3x3 = Tile3x3([false; 9]);

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
                    tile.set(i, !is_white);

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
