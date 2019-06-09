use std::fs;

use crate::components::{Color, Tile};
use crate::components::{BLACK, DEFAULT_TILE_SIZE};

pub const BATCH_SIZE: usize = DEFAULT_TILE_SIZE * DEFAULT_TILE_SIZE;

pub struct Batch {
  pub tile: Tile,
  pub colors: [Color; BATCH_SIZE],
}

#[derive(Debug, PartialEq)]
pub struct Canvas {
  canvas: Vec<Color>,
  height: usize,
  width: usize,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      canvas: vec![BLACK; width * height],
      height: height,
      width: width,
    }
  }

  pub fn from_batches(width: usize, height: usize, batches: &[Batch]) -> Self {
    let mut canvas = vec![BLACK; width * height];

    for batch in batches.iter() {
      let Batch { tile, colors } = batch;
      for y in tile.y_start..tile.y_end {
        for x in tile.x_start..tile.x_end {
          let canvas_y = y as usize;
          let canvas_x = x as usize;
          let data_y = (y - tile.y_start) as usize;
          let data_x = (x - tile.x_start) as usize;
          canvas[canvas_y * width + canvas_x] = colors[data_y * DEFAULT_TILE_SIZE + data_x];
        }
      }
    }

    Self {
      canvas,
      height: height,
      width: width,
    }
  }

  pub fn write(&mut self, x: usize, y: usize, color: &Color) {
    self.canvas[y * self.width + x] = *color;
  }

  fn as_ppm(&self) -> String {
    let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
    if self.canvas.len() == 0 {
      return ppm;
    };

    let mut line: String = self.canvas.first().unwrap().to_channels_array().join(" ");
    let mut count = 1;
    for color in self.canvas.iter().skip(1) {
      for channel in color.to_channels_array().iter() {
        if line.len() + channel.len() >= 70 || count >= self.width {
          if count >= self.width {
            count = 0;
          }
          line.push('\n');
          ppm.push_str(&line);
          line = channel.to_string();
        } else {
          line.push(' ');
          line.push_str(&channel);
        }
      }
      count += 1;
    }
    ppm.push_str(&line);
    ppm.push('\n');
    ppm
  }

  pub fn export_ppm(&self, file: &String) {
    fs::write(file, self.as_ppm()).expect("Unable to write file");
  }
}

#[cfg(test)]
mod tests {
  mod methods {
    use super::super::Canvas;
    use crate::components::Color;

    #[test]
    fn constructor() {
      let width = 10;
      let height = 20;
      let expected = Canvas {
        canvas: vec![Color::new(0.0, 0.0, 0.0); width * height],
        height: height,
        width: width,
      };
      assert_eq!(expected, Canvas::new(width, height));
    }

    #[test]
    fn write() {
      let mut c = Canvas::new(10, 20);
      let red = Color::new(1.0, 0.0, 0.0);
      let x = 2;
      let y = 3;
      c.write(x, y, &red);
      assert_eq!(c.canvas[y * c.width + x], red);
    }

    mod as_ppm {
      use super::{Canvas, Color};

      #[test]
      fn header() {
        assert_eq!(
          String::from("P3\n5 3\n255"),
          Canvas::new(5, 3)
            .as_ppm()
            .lines()
            .take(3)
            .collect::<Vec<_>>()
            .join("\n")
        );
      }

      #[test]
      fn body() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write(0, 0, &c1);
        c.write(2, 1, &c2);
        c.write(4, 2, &c3);
        let body: String = [
          "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
          "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
          "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
        ]
        .join("\n");
        assert_eq!(
          body,
          c.as_ppm().lines().skip(3).collect::<Vec<_>>().join("\n")
        );
      }

      #[test]
      fn body_splitting() {
        let mut canvas = Canvas::new(10, 2);
        canvas.canvas = vec![Color::new(1.0, 0.8, 0.6); canvas.width * canvas.height];
        let body: String = [
          "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
          "153 255 204 153 255 204 153 255 204 153 255 204 153",
          "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
          "153 255 204 153 255 204 153 255 204 153 255 204 153",
        ]
        .join("\n");
        assert_eq!(
          body,
          canvas
            .as_ppm()
            .lines()
            .skip(3)
            .collect::<Vec<_>>()
            .join("\n")
        );
      }

      #[test]
      fn endline() {
        assert_eq!('\n', Canvas::new(5, 3).as_ppm().pop().unwrap());
      }
    }
  }
}
