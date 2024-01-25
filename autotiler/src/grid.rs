use crate::tile::{Tile3x3, C_IDX, E_IDX, N_IDX, NE_IDX, NW_IDX, S_IDX, SE_IDX, SW_IDX, W_IDX};
use rand::prelude::*;
use crate::point::Point;
use crate::rect::Rect;


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

    pub fn get_pt(&self, pt: &Point) -> Option<Tile3x3> {
        let idx = self.idx(pt)?;
        Some(self.data[idx].clone())
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

pub fn grid_strip_invalid_in_place(tile_grid: &mut RectVec) {
    for y in 0..tile_grid.bounds.h {
        for x in 0..tile_grid.bounds.w {
            let pos = Point { x, y };

            let mut tile = tile_grid.get_pt(&pos).expect("a tile at this position");

            if tile.get(C_IDX) == false {
                continue;
            }


            // check diagonal neighbour for contiguous fill cases
            if tile.get(NW_IDX) {
                if let Some(neighbour) = tile_grid.get_pt(&pos.north_west()) {
                    tile.set(NW_IDX, neighbour.get(SE_IDX));
                }
            }

            if tile.get(NE_IDX) {
                if let Some(neighbour) = tile_grid.get_pt(&pos.north_east()) {
                    tile.set(NE_IDX, neighbour.get(SW_IDX));
                }
            }

            if tile.get(SW_IDX) {
                if let Some(neighbour) = tile_grid.get_pt(&pos.south_west()) {
                    tile.set(SW_IDX, neighbour.get(NE_IDX));
                }
            }

            if tile.get(SE_IDX) {
                if let Some(neighbour) = tile_grid.get_pt(&pos.south_east()) {
                    tile.set(SE_IDX, neighbour.get(NW_IDX));
                }
            }

            // clear out invalid pixels
            if let Some(neighbour) = tile_grid.get_pt(&pos.north()) {
                tile.set(N_IDX, tile.get(N_IDX) & neighbour.get(C_IDX) & neighbour.get(S_IDX) & tile.get(C_IDX));

                tile.set(NW_IDX, tile.get(NW_IDX) & neighbour.get(C_IDX) & neighbour.get(SW_IDX) & tile.get(C_IDX));
                tile.set(NE_IDX, tile.get(NE_IDX) & neighbour.get(C_IDX) & neighbour.get(SE_IDX) & tile.get(C_IDX));
            }

            if let Some(neighbour) = tile_grid.get_pt(&pos.west()) {
                tile.set(W_IDX, tile.get(W_IDX) & neighbour.get(C_IDX) & neighbour.get(E_IDX) & tile.get(C_IDX));

                tile.set(NW_IDX, tile.get(NW_IDX) & neighbour.get(C_IDX) & neighbour.get(NE_IDX) & tile.get(C_IDX));
                tile.set(SW_IDX, tile.get(SW_IDX) & neighbour.get(C_IDX) & neighbour.get(SE_IDX) & tile.get(C_IDX));
            }

            if let Some(neighbour) = tile_grid.get_pt(&pos.east()) {
                tile.set(E_IDX, tile.get(E_IDX) & neighbour.get(C_IDX) & neighbour.get(W_IDX) & tile.get(C_IDX));

                tile.set(NE_IDX, tile.get(NE_IDX) & neighbour.get(C_IDX) & neighbour.get(NW_IDX) & tile.get(C_IDX));
                tile.set(SE_IDX, tile.get(SE_IDX) & neighbour.get(C_IDX) & neighbour.get(SW_IDX) & tile.get(C_IDX));
            }

            if let Some(neighbour) = tile_grid.get_pt(&pos.south()) {
                tile.set(S_IDX, tile.get(S_IDX) & neighbour.get(C_IDX) & neighbour.get(N_IDX) & tile.get(C_IDX));

                tile.set(SW_IDX, tile.get(SW_IDX) & neighbour.get(C_IDX) & neighbour.get(NW_IDX) & tile.get(C_IDX));
                tile.set(SE_IDX, tile.get(SE_IDX) & neighbour.get(C_IDX) & neighbour.get(NE_IDX) & tile.get(C_IDX));
            }

            tile_grid.set_pt(&pos, tile);
        }
    }
}


