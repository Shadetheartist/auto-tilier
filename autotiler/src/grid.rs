use crate::tile::{Tile3x3, C_PT, E_PT, N_PT, NE_PT, NW_PT, S_PT, SE_PT, SW_PT, W_PT};
use rand::prelude::*;
use crate::point::Point;
use crate::rect::Rect;


#[derive(Clone)]
pub struct RectVec {
    data: Vec<Tile3x3>,
    pub bounds: Rect,
}

impl RectVec {
    pub fn new(bounds: Rect) -> Self {
        let data = vec![Tile3x3::default(); (bounds.w * bounds.h) as usize];

        Self {
            data,
            bounds,
        }
    }

    pub fn idx(&self, pt: &Point) -> Option<usize> {
        if self.bounds.contains(pt) {
            Some((pt.y * self.bounds.w + pt.x) as usize)
        } else {
            None
        }
    }

    pub fn get_pt(&self, pt: &Point) -> Option<&Tile3x3> {
        let idx = self.idx(pt)?;
        Some(&self.data[idx])
    }

    pub fn set_pt(&mut self, pt: &Point, value: Tile3x3) {
        if let Some(idx) = self.idx(pt) {
            self.data[idx] = value
        }
    }

    pub fn iter_enumerate(&self) -> impl Iterator<Item=(Point, &Tile3x3)> {
        self.data.iter().enumerate().map(|(index, tile)| {
            let x = index as i32 % self.bounds.w;
            let y = index as i32 / self.bounds.w;
            (Point { x, y }, tile)
        })
    }
}


fn random_tile_from_tile_set(tile_set: &Vec<Tile3x3>) -> Tile3x3 {
    let mut rng = thread_rng();
    tile_set.choose(&mut rng).cloned().unwrap()
}

pub fn generate_test_grid(tile_set: &Vec<Tile3x3>, width: u32, height: u32) -> RectVec {
    let mut grid = RectVec::new(Rect::new(0, 0, width as i32, height as i32));

    for y in 0..height {
        for x in 0..width {
            grid.set_pt(
                &Point { x: x as i32, y: y as i32 },
                random_tile_from_tile_set(tile_set),
            )
        }
    }

    return grid;
}

pub fn grid_strip_invalid(tile_grid: &RectVec) -> RectVec {
    let mut stripped = RectVec::new(tile_grid.bounds.clone());

    for (pos, tile) in tile_grid.iter_enumerate() {

        let mut tile = tile.clone();

        if tile.get(&C_PT) == false {
            continue;
        }

        // check diagonal neighbour for contiguous fill cases
        if tile.get(&NW_PT) {
            if let Some(neighbour) = tile_grid.get_pt(&pos.north_west()) {
                tile.set(&NW_PT, neighbour.get(&SE_PT));
            }
        }

        if tile.get(&NE_PT) {
            if let Some(neighbour) = tile_grid.get_pt(&pos.north_east()) {
                tile.set(&NE_PT, neighbour.get(&SW_PT));
            }
        }

        if tile.get(&SW_PT) {
            if let Some(neighbour) = tile_grid.get_pt(&pos.south_west()) {
                tile.set(&SW_PT, neighbour.get(&NE_PT));
            }
        }

        if tile.get(&SE_PT) {
            if let Some(neighbour) = tile_grid.get_pt(&pos.south_east()) {
                tile.set(&SE_PT, neighbour.get(&NW_PT));
            }
        }

        // clear out invalid pixels
        if let Some(neighbour) = tile_grid.get_pt(&pos.north()) {
            tile.set(&N_PT, tile.get(&N_PT) & neighbour.get(&C_PT) & neighbour.get(&S_PT) & tile.get(&C_PT));

            tile.set(&NW_PT, tile.get(&NW_PT) & neighbour.get(&C_PT) & neighbour.get(&SW_PT) & tile.get(&C_PT));
            tile.set(&NE_PT, tile.get(&NE_PT) & neighbour.get(&C_PT) & neighbour.get(&SE_PT) & tile.get(&C_PT));
        }

        if let Some(neighbour) = tile_grid.get_pt(&pos.west()) {
            tile.set(&W_PT, tile.get(&W_PT) & neighbour.get(&C_PT) & neighbour.get(&E_PT) & tile.get(&C_PT));

            tile.set(&NW_PT, tile.get(&NW_PT) & neighbour.get(&C_PT) & neighbour.get(&NE_PT) & tile.get(&C_PT));
            tile.set(&SW_PT, tile.get(&SW_PT) & neighbour.get(&C_PT) & neighbour.get(&SE_PT) & tile.get(&C_PT));
        }

        if let Some(neighbour) = tile_grid.get_pt(&pos.east()) {
            tile.set(&E_PT, tile.get(&E_PT) & neighbour.get(&C_PT) & neighbour.get(&W_PT) & tile.get(&C_PT));

            tile.set(&NE_PT, tile.get(&NE_PT) & neighbour.get(&C_PT) & neighbour.get(&NW_PT) & tile.get(&C_PT));
            tile.set(&SE_PT, tile.get(&SE_PT) & neighbour.get(&C_PT) & neighbour.get(&SW_PT) & tile.get(&C_PT));
        }

        if let Some(neighbour) = tile_grid.get_pt(&pos.south()) {
            tile.set(&S_PT, tile.get(&S_PT) & neighbour.get(&C_PT) & neighbour.get(&N_PT) & tile.get(&C_PT));

            tile.set(&SW_PT, tile.get(&SW_PT) & neighbour.get(&C_PT) & neighbour.get(&NW_PT) & tile.get(&C_PT));
            tile.set(&SE_PT, tile.get(&SE_PT) & neighbour.get(&C_PT) & neighbour.get(&NE_PT) & tile.get(&C_PT));
        }

        stripped.set_pt(&pos, tile);
    }

    stripped
}


