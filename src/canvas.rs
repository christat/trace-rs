use std::fs;
use super::color::{Color};

#[derive(Debug, PartialEq)]
pub struct Canvas {
  canvas: Vec<Color>,
  height: usize,
  width: usize
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      canvas: vec![Color::new(0.0, 0.0, 0.0); width * height],
      height: height,
      width: width
    }
  }

  pub fn write(&mut self, x: usize, y: usize, color: &Color) {
    self.canvas[y * self.width + x] = *color;
  }

  fn as_ppm(&self) -> String {
    let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
    if self.canvas.len() == 0 { return ppm };

    let mut line: String = color_to_channels(self.canvas.first().unwrap()).join(" ");
    let mut count = 1;
    for color in self.canvas.iter().skip(1) {
      for channel in color_to_channels(color) {
        if line.len() + channel.len() >= 70 || count >= self.width {
          if count >= self.width {
            count = 0;
          }
          line.push('\n');
          ppm.push_str(&line);
          line = channel;
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

fn color_to_channels(color: &Color) -> Vec<String> {
  let r = f32::min(255.0, f32::max(0.0, (color.r() * 255.0).round())) as u8;
  let g = f32::min(255.0, f32::max(0.0, (color.g() * 255.0).round())) as u8;
  let b = f32::min(255.0, f32::max(0.0, (color.b() * 255.0).round())) as u8;
  vec![r.to_string(), g.to_string(), b.to_string()]
}

#[cfg(test)]
mod tests {
  use super::Color;
  use super::Canvas;
  use super::color_to_channels;

  #[test]
  fn implements_color_to_channels() {
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);
    assert_eq!("255 0 0".split_whitespace().collect::<Vec<_>>(), color_to_channels(&c1));
    assert_eq!("0 128 0".split_whitespace().collect::<Vec<_>>(), color_to_channels(&c2));
    assert_eq!("0 0 255".split_whitespace().collect::<Vec<_>>(), color_to_channels(&c3));
  }

  #[test]
  fn implements_constructor() {
    let width = 10;
    let height = 20;
    let expected = Canvas {
      canvas: vec![Color::new(0.0, 0.0, 0.0); width * height],
      height: height,
      width: width
    };
    assert_eq!(expected, Canvas::new(width, height));
  }

  #[test]
  fn implements_write() {
    let mut c = Canvas::new(10, 20);
    let red = Color::new(1.0, 0.0, 0.0);
    let x = 2;
    let y = 3;
    c.write(x, y, &red);
    assert_eq!(c.canvas[y * c.width + x], red);
  }

  #[test]
  fn implements_as_ppm_header() {
    assert_eq!(
      String::from("P3\n5 3\n255"),
      Canvas::new(5, 3).as_ppm().lines().take(3).collect::<Vec<_>>().join("\n")
    );
  }

  #[test]
  fn implements_as_ppm_body() {
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
      "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
    ].join("\n");
    assert_eq!(body, c.as_ppm().lines().skip(3).collect::<Vec<_>>().join("\n"));
  }

  #[test]
  fn implements_as_ppm_body_splitting() {
    let mut canvas = Canvas::new(10, 2);
    canvas.canvas = vec![Color::new(1.0, 0.8, 0.6); canvas.width * canvas.height];
    let body: String = [
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
      "153 255 204 153 255 204 153 255 204 153 255 204 153",
      "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
      "153 255 204 153 255 204 153 255 204 153 255 204 153",
    ].join("\n");
    assert_eq!(body, canvas.as_ppm().lines().skip(3).collect::<Vec<_>>().join("\n"));
  }

  #[test]
  fn implements_as_ppm_endline() {
    assert_eq!('\n', Canvas::new(5, 3).as_ppm().pop().unwrap());
  }
}