use rand::prelude::SliceRandom;
use rand::thread_rng;
use crate::point::Point;
use crate::rect::Rect;
use crate::tile::{C_IDX, E_IDX, N_IDX, NE_IDX, NW_IDX, S_IDX, SE_IDX, SW_IDX, Tile3x3, W_IDX};

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<bool>,
    pub bounds: Rect,
}

pub type MatrixTile = [bool];

impl Matrix {
    pub fn new(bounds: Rect) -> Self {
        let bounds = Rect::new(bounds.x, bounds.y, bounds.w * 3, bounds.h * 3);
        let data = vec![false; (bounds.w * bounds.h) as usize];
        Self {
            data,
            bounds,
        }
    }

    pub fn idx_tile(&self, pt: &Point) -> Option<usize> {
        let idx = (pt.y * self.bounds.w * 3 + pt.x * 9) as usize;

        Some(idx)
    }

    pub fn idx(&self, pt: &Point) -> Option<usize> {
        if self.bounds.contains(pt) {
            Some((pt.y * self.bounds.w + pt.x) as usize)
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
            let x = index as i32 % self.bounds.w;
            let y = index as i32 / self.bounds.w;
            (Point { x, y }, bit)
        })
    }

    pub fn iter_tiles_enumerate(&self) -> impl Iterator<Item=(Point, &MatrixTile)> {
        self.data.iter().enumerate().map(|(index, bit)| {
            let x = index as i32 % self.bounds.w;
            let y = index as i32 / self.bounds.w;
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
        let mut matrix = Matrix::new(Rect::new(0,0,self.bounds.w/3, self.bounds.h/3));

        for y in 0..self.bounds.h / 3 {
            for x in 0..self.bounds.w / 3 {
                let pt = Point {x, y };
                let tile = self.tile(&pt).unwrap();
                matrix.tile_mut(&pt).unwrap().copy_from_slice(tile);
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

