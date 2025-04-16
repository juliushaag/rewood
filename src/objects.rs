use std::fmt::Display;


pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8
}


/* 2D Quad starting in the upper left corner*/

pub struct Quad2D {
  pub x: u32, 
  pub y: u32,

  pub width: u32,
  pub height: u32,

  pub color : Color 
}

impl Display for Quad2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quad2D {{ x: {}, y: {}, width: {}, height: {}, color: rgba({}, {}, {}, {}) }}",
            self.x, self.y, self.width, self.height, self.color.r, self.color.g, self.color.b, self.color.a
        )
    }
}

impl Quad2D {

  pub fn new(x: u32, y: u32, width: u32, height: u32, color : Color) -> Self {
    Self { x, y, width, height, color }
  }
}