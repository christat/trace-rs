use specs::{Entities, Read, System, WriteStorage};

use crate::components::{Tile, DEFAULT_TILE_SIZE};
use crate::resources::Camera;

/// Splits a given scene's representable grid into tiles of aggregated pixels;
/// This helps the render system achieve greater data locality per thread.
/// Each tile uses 16x16 pixels unless the image is not exactly divisible.
/// In the latter case, varied sizes of boundary tiles are generated.
pub struct TilingSystem;

impl<'a> System<'a> for TilingSystem {
  type SystemData = (Entities<'a>, Read<'a, Camera>, WriteStorage<'a, Tile>);

  fn run(&mut self, (entities, camera, mut tiles): Self::SystemData) {
    let tile_size = DEFAULT_TILE_SIZE as u16;
    for y_start in (0..camera.height).step_by(DEFAULT_TILE_SIZE) {
      for x_start in (0..camera.width).step_by(DEFAULT_TILE_SIZE) {
        let x_span = x_start + tile_size;
        let y_span = y_start + tile_size;
        let x_end = if x_span <= camera.width {
          x_span
        } else {
          camera.width
        };
        let y_end = if y_span <= camera.height {
          y_span
        } else {
          camera.height
        };
        tiles
          .insert(
            entities.create(),
            Tile {
              x_start,
              x_end,
              y_start,
              y_end,
            },
          )
          .unwrap();
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::TilingSystem;
  use crate::components::Tile;
  use crate::resources::Camera;
  use specs::{DispatcherBuilder, Join, Read, ReadStorage, System, World};

  type ExpectedTiles = Vec<Tile>;
  struct AssertTiles;
  impl<'a> System<'a> for AssertTiles {
    type SystemData = (Read<'a, ExpectedTiles>, ReadStorage<'a, Tile>);

    fn run(&mut self, data: Self::SystemData) {
      let (expected_tiles, tiles) = data;

      let mut tile_count = 0;
      for tile in tiles.join() {
        assert!(expected_tiles.contains(tile));
        tile_count += 1;
      }
      assert_eq!(expected_tiles.len(), tile_count);
    }
  }

  fn test_tiling<'a, 'b>(camera: Camera, expected_tiles: ExpectedTiles) {
    let mut world = World::new();
    world.add_resource(camera);
    world.add_resource(expected_tiles);
    world.register::<Tile>();

    let mut dispatcher = DispatcherBuilder::new()
      .with(TilingSystem, "tiling", &[])
      .with(AssertTiles, "assert_tiles", &["tiling"])
      .build();

    dispatcher.dispatch(&world.res);
    world.maintain();
  }

  #[test]
  fn divisible_resolution() {
    let mut camera = Camera::default();
    camera.width = 32;
    camera.height = 32;

    let expected_tiles = vec![
      Tile {
        x_start: 0,
        x_end: 16,
        y_start: 0,
        y_end: 16,
      },
      Tile {
        x_start: 16,
        x_end: 32,
        y_start: 0,
        y_end: 16,
      },
      Tile {
        x_start: 0,
        x_end: 16,
        y_start: 16,
        y_end: 32,
      },
      Tile {
        x_start: 16,
        x_end: 32,
        y_start: 16,
        y_end: 32,
      },
    ];

    test_tiling(camera, expected_tiles);
  }

  #[test]
  fn non_divisible_resolution() {
    let mut camera = Camera::default();
    camera.width = 39;
    camera.height = 19;

    let expected_tiles = vec![
      Tile {
        x_start: 0,
        x_end: 16,
        y_start: 0,
        y_end: 16,
      },
      Tile {
        x_start: 16,
        x_end: 32,
        y_start: 0,
        y_end: 16,
      },
      Tile {
        x_start: 32,
        x_end: 39,
        y_start: 0,
        y_end: 16,
      },
      Tile {
        x_start: 0,
        x_end: 16,
        y_start: 16,
        y_end: 19,
      },
      Tile {
        x_start: 16,
        x_end: 32,
        y_start: 16,
        y_end: 19,
      },
      Tile {
        x_start: 32,
        x_end: 39,
        y_start: 16,
        y_end: 19,
      },
    ];

    test_tiling(camera, expected_tiles);
  }
}
