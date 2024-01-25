use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::point::Point;
use crate::rect::Rect;
use crate::tile::{C_IDX, E_IDX, N_IDX, NE_IDX, NW_IDX, S_IDX, SE_IDX, SW_IDX, Tile3x3, W_IDX};

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<bool>,
    pub px_bounds: Rect,
    pub tile_bounds: Rect,
}

pub type MatrixTile = [bool];

impl Matrix {
    pub fn new(bounds: Rect) -> Self {
        let px_bounds = Rect::new(bounds.x, bounds.y, bounds.w * 3, bounds.h * 3);
        let data = vec![false; (px_bounds.w * px_bounds.h) as usize];
        Self {
            data,
            px_bounds,
            tile_bounds: bounds,
        }
    }

    pub fn idx_tile(&self, pt: &Point) -> Option<usize> {

        if self.tile_bounds.contains(pt) {
            let idx = (pt.y * self.px_bounds.w * 3 + pt.x * 9) as usize;
            Some(idx)
        } else {
            None
        }


    }

    pub fn idx(&self, pt: &Point) -> Option<usize> {
        if self.px_bounds.contains(pt) {
            Some((pt.y * self.px_bounds.w + pt.x) as usize)
        } else {
            None
        }
    }

    pub fn get_pt(&self, pt: &Point) -> Option<bool> {
        let idx = self.idx(pt)?;
        Some(self.data[idx])
    }

    pub fn set_pt(&mut self, pt: &Point, value: bool) {
        if let Some(idx) = self.idx(pt) {
            self.data[idx] = value
        }
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item=(Point, &bool)> {
        self.data.iter().enumerate().map(|(index, bit)| {
            let x = index as i32 % self.px_bounds.w;
            let y = index as i32 / self.px_bounds.w;
            (Point { x, y }, bit)
        })
    }

    pub fn iter_tiles_enumerate(&self) -> impl Iterator<Item=(Point, &MatrixTile)> {
        self.data.iter().enumerate().map(|(index, bit)| {
            let x = index as i32 % self.px_bounds.w;
            let y = index as i32 / self.px_bounds.w;
            let tile_pt = Point { x: x / 3, y: y / 3 };
            let tile = self.tile(&tile_pt).unwrap();
            (tile_pt, tile)
        })
    }

    pub fn tile(&self, pt: &Point) -> Option<&MatrixTile> {
        let idx = self.idx_tile(&pt)?;

        if idx + 9 > self.data.len() {
            return None;
        }

        Some(&self.data[idx..idx + 9])
    }

    pub fn tile_mut(&mut self, pt: &Point) -> Option<&mut MatrixTile> {
        let idx = self.idx_tile(&pt)?;

        if idx + 9 > self.data.len() {
            return None;
        }

        Some(&mut self.data[idx..idx + 9])
    }

    pub fn strip_invalid(&self) -> Matrix {
        let mut matrix = self.clone();

        for y in 0..matrix.px_bounds.h / 3 {
            for x in 0..matrix.px_bounds.w / 3 {
                let pos = Point {x, y };
                let tile = matrix.tile_mut(&pos).unwrap();

                if tile[C_IDX] == false {
                    continue;
                }

                // check diagonal neighbour for contiguous fill cases
                if tile[NW_IDX] {
                    if let Some(neighbour) = self.tile(&pos.north_west()) {
                        tile[NW_IDX] = neighbour[SE_IDX];
                    }
                }

                if tile[NE_IDX] {
                    if let Some(neighbour) = self.tile(&pos.north_east()) {
                        tile[NE_IDX] = neighbour[SW_IDX];
                    }
                }

                if tile[SW_IDX] {
                    if let Some(neighbour) = self.tile(&pos.south_west()) {
                        tile[SW_IDX] = neighbour[NE_IDX];
                    }
                }

                if tile[SE_IDX] {
                    if let Some(neighbour) = self.tile(&pos.south_east()) {
                        tile[SE_IDX] = neighbour[NW_IDX];
                    }
                }

                // clear out invalid pixels
                if let Some(neighbour) = self.tile(&pos.north()) {
                    tile[N_IDX] = tile[N_IDX] & neighbour[C_IDX] & neighbour[S_IDX] & tile[C_IDX];

                    tile[NW_IDX] = tile[NW_IDX] & neighbour[C_IDX] & neighbour[SW_IDX] & tile[C_IDX];
                    tile[NE_IDX] = tile[NE_IDX] & neighbour[C_IDX] & neighbour[SE_IDX] & tile[C_IDX];
                }

                if let Some(neighbour) = self.tile(&pos.west()) {
                    tile[W_IDX] = tile[W_IDX] & neighbour[C_IDX] & neighbour[E_IDX] & tile[C_IDX];

                    tile[NW_IDX] = tile[NW_IDX] & neighbour[C_IDX] & neighbour[NE_IDX] & tile[C_IDX];
                    tile[SW_IDX] = tile[SW_IDX] & neighbour[C_IDX] & neighbour[SE_IDX] & tile[C_IDX];
                }

                if let Some(neighbour) = self.tile(&pos.east()) {
                    tile[E_IDX] = tile[E_IDX] & neighbour[C_IDX] & neighbour[W_IDX] & tile[C_IDX];

                    tile[NE_IDX] = tile[NE_IDX] & neighbour[C_IDX] & neighbour[NW_IDX] & tile[C_IDX];
                    tile[SE_IDX] = tile[SE_IDX] & neighbour[C_IDX] & neighbour[SW_IDX] & tile[C_IDX];
                }

                if let Some(neighbour) = self.tile(&pos.south()) {
                    tile[S_IDX] = tile[S_IDX] & neighbour[C_IDX] & neighbour[N_IDX] & tile[C_IDX];

                    tile[SW_IDX] = tile[SW_IDX] & neighbour[C_IDX] & neighbour[NW_IDX] & tile[C_IDX];
                    tile[SE_IDX] = tile[SE_IDX] & neighbour[C_IDX] & neighbour[NE_IDX] & tile[C_IDX];
                }
            }
        }

        matrix
    }
}

fn random_tile_from_tile_set(tile_set: &Vec<Tile3x3>) -> Tile3x3 {
    let mut rng = thread_rng();
    tile_set.choose(&mut rng).cloned().unwrap()
}

pub fn generate_random_matrix(tile_set: &Vec<Tile3x3>, width: u32, height: u32) -> Matrix {
    let mut matrix = Matrix::new(Rect::new(0, 0, width as i32, height as i32));

    for y in 0..height {
        for x in 0..width {
            let pt = Point { x: x as i32, y: y as i32 };
            let random_tile = random_tile_from_tile_set(&tile_set);
            let tile_slice = matrix.tile_mut(&pt).unwrap();
            tile_slice.copy_from_slice(&random_tile.0[..])
        }
    }

    return matrix;
}

