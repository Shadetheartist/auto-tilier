use crate::tile::*;
use crate::point::Point;
use crate::rect::Rect;


pub fn auto_tile(tile_grid: &mut Vec<Vec<Tile3x3>>) {
    // solve_tile_grid(&tile_grid)
}

pub fn solve_tile_grid(tile_grid: &[Vec<Tile3x3>]) -> Vec<Vec<Tile3x3>> {
    let mut solved_grid = Vec::new();

    let bounds = Rect::new(
        0,
        0,
        tile_grid.first().expect("grid to have a row").len() as i32,
        tile_grid.len() as i32
    );

    for (y, row) in tile_grid.iter().enumerate() {
        let mut solved_row = Vec::new();

        for (x, tile) in row.iter().enumerate() {
            solved_row.push(solve_tile(tile_grid, &bounds, &tile, Point { x: x as i32, y: y as i32 }));
        }

        solved_grid.push(solved_row);
    }

    solved_grid
}


#[allow(dead_code, unused_variables)]
fn solve_tile(tile_grid: &[Vec<Tile3x3>], bounds: &Rect, tile: &Tile3x3, pos: Point) -> Tile3x3 {
    let tile = tile.clone();


    tile
}

