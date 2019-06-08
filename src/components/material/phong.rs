use super::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Phong {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub shininess: f32
}

impl Phong {
    pub fn new(color: Color, ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess
        }
    }
}

impl Default for Phong {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Phong};

    #[test]
    fn constructor() {
        let mat = Phong::default();
        assert_eq!(Phong {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }, mat);

        let mat = Phong::new(Color::new(1.0, 0.0, 0.0), 0.0, 1.0, 2.0, 3.0);
        assert_eq!(Phong {
            color: Color::new(1.0, 0.0, 0.0),
            ambient: 0.0,
            diffuse: 1.0,
            specular: 2.0,
            shininess: 3.0
        }, mat);
    }
}