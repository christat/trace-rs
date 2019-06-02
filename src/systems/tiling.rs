use specs::{Read, System, Write};
use crate::resources::{Camera, Tiles, Tile, DEFAULT_TILE_SIZE};

/// Splits a given scene's representable grid into tiles of aggregated pixels;
/// This helps the render system achieve greater data locality per thread.
/// Each tile uses 16x16 pixels unless the image is not exactly divisible.
/// In the latter case, varied sizes of boundary tiles are generated.
pub struct Tiling;

impl<'a> System<'a> for Tiling {
  type SystemData = (
    Read<'a, Camera>,
    Write<'a, Tiles>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (camera, mut tiles) = data;

    let height = camera.v_size;
    let width = camera.h_size;
    
    for y in (0..height).step_by(DEFAULT_TILE_SIZE) {
      for x in (0..width).step_by(DEFAULT_TILE_SIZE) {
        tiles.mut_vector().push(Tile { x, y });
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use specs::{DispatcherBuilder, Read, System, World};
  use crate::resources::{Camera, Tiles, Tile};
  use super::Tiling;

  type ExpectedTiles = Tiles;
  struct AssertTiles;
  impl<'a> System<'a> for AssertTiles {
    type SystemData = (Read<'a, ExpectedTiles>, Read<'a, Tiles>);

    fn run(&mut self, data: Self::SystemData) {
      let (expected_tiles, tiles) = data;
      assert_eq!(expected_tiles.slice(), tiles.slice());
    }
  }

  fn test_tiling<'a, 'b>(camera: Camera, expected_tiles: ExpectedTiles) {
    let mut world = World::new();
    world.add_resource(camera);
    world.add_resource(expected_tiles);
    world.add_resource(Tiles::default());
    
    let mut dispatcher = DispatcherBuilder::new()
      .with(Tiling, "tiling", &[])
      .with(AssertTiles, "assert_tiles", &["tiling"])
      .build();

    dispatcher.dispatch(&world.res);
    world.maintain();
  }

  #[test]
  fn tiling_divisible() {
    let mut camera = Camera::default();
    camera.h_size = 32;
    camera.v_size = 32;

    let expected_tiles = ExpectedTiles::new(
      vec!(
        Tile { x: 0, y: 0 },
        Tile { x: 16, y: 0 },
        Tile { x: 0, y: 16 },
        Tile { x: 16, y: 16 }
      )
    );

    test_tiling(camera, expected_tiles);
  }

  #[test]
  fn tiling_non_divisible() {
    let mut camera = Camera::default();
    camera.h_size = 39;
    camera.v_size = 19;

    let expected_tiles = ExpectedTiles::new(
      vec!(
        Tile { x: 0, y: 0 },
        Tile { x: 16, y: 0 },
        Tile { x: 32, y: 0 },
        Tile { x: 0, y: 16 },
        Tile { x: 16, y: 16 },
        Tile { x: 32, y: 16 }
      )
    );

    test_tiling(camera, expected_tiles);
  }
}