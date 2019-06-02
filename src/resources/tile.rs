
/// Fixed default size (in pixels) for each render tile.
/// 
/// Based off of pbrt's own tiling system:
/// http://www.pbr-book.org/3ed-2018/Introduction/pbrt_System_Overview.html#TheMainRenderingLoop
pub const DEFAULT_TILE_SIZE: usize = 16;

/// Core resource for the tracing System.
/// 
/// Separates the output into rectangular areas
/// where computation will be localized on a per-thread basis.
#[derive(Default)]
pub struct Tiles(Vec<Tile>);

impl Tiles {
  pub fn new(tiles: Vec<Tile>) -> Tiles {
    Self(tiles)
  }

  pub fn mut_vector(&mut self) -> &mut Vec<Tile> {
    &mut self.0
  }

  pub fn slice(&self) -> &[Tile] {
    &self.0
  }
}

/// pair of (x, y) representing which sub-region of the image to trace.
/// Covers an area of DEFAULT_TILE_SIZE starting from (x, y).
///
/// It is assumed the tracing system handles boundary tiles whose 
/// size is lesser than the default.
#[derive(Debug, PartialEq)]
pub struct Tile {
  pub x: u16,
  pub y: u16
}