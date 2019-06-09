mod intersection;
mod lighting;

use rayon::prelude::*;
use specs::{ParJoin, Read, ReadStorage, System};

use crate::{Batch, Canvas};
use crate::components::{Material, Position, Shape, Tile, Transform};
use crate::resources::{Camera, Lights};

use crate::BATCH_SIZE;
use crate::components::{BLACK, DEFAULT_TILE_SIZE};

pub use intersection::{HitMetadata, IntersectionRecord, IntersectionSubsystem};
pub use lighting::LightingSubsystem;

/// Raytracing root system.
/// Processes tiles in parallel, calling the intersection
/// and lighting subsystems, and generating a Batch per
/// processed Tile.
pub struct TracingSystem;

impl<'a> System<'a> for TracingSystem {
  type SystemData = (
    Read<'a, Camera>,
    Read<'a, Lights>,
    ReadStorage<'a, Tile>,
    ReadStorage<'a, Position>,
    ReadStorage<'a, Shape>,
    ReadStorage<'a, Transform>,
    ReadStorage<'a, Material>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (camera, lights, tiles, positions, shapes, transforms, materials) = data;

    let par_iter = tiles.par_join().map(|tile| {
      let mut batch = Batch {
        tile: *tile,
        colors: [BLACK; BATCH_SIZE],
      };

      for y in tile.y_start..tile.y_end {
        for x in tile.x_start..tile.x_end {
          let ray = camera.ray_for_pixel(x, y);
          let color = IntersectionSubsystem::run(&ray, &lights, &positions, &shapes, &transforms, &materials);
          if color.is_some() {
            let index = (y - tile.y_start) as usize * DEFAULT_TILE_SIZE + (x - tile.x_start) as usize;
            batch.colors[index] = color.unwrap();
          }
        }
      }

      batch
    });

    let batches: Vec<Batch> = par_iter.collect();
    Canvas::from_batches(camera.width as usize, camera.height as usize, &batches).export_ppm(&String::from("ecs.ppm"));
  }
}
