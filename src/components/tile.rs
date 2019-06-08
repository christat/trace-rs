use specs::{Component, VecStorage};

/// Fixed default size (in pixels) for each render tile.
/// 
/// Based off of pbrt's own tiling system:
/// http://www.pbr-book.org/3ed-2018/Introduction/pbrt_System_Overview.html#TheMainRenderingLoop
pub const DEFAULT_TILE_SIZE: usize = 16;

/// pair of (x, y) points representing a sub-region of the image to trace (i.e. a tile).
/// Covers an area of up to DEFAULT_TILE_SIZE x DEFAULT_TILE_SIZE starting from (x_start, y_start).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tile {
  pub x_start: u16,
  pub x_end: u16,
  pub y_start: u16,
  pub y_end: u16
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}