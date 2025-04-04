


pub enum Size {

  // Fixed scales
  Pixel(u32),
  
  // Parent relatice scales
  Max,
  Relative(f32),
  MaxMinRelative { min: f32, max: f32 },
  
  // Child relative scaling
  Content,


  Overflow,
  

  Same,
}

enum LayoutDirection {
  Horizontal,
  Vertical
}

enum LayoutAlignment {
  Start,
  End,
  Center
}

pub struct Layout {
  pub width : u32,
  pub height : u32,
}

pub enum LayoutError {
  InsufficientSpace,
}

struct LayoutComponent {
  pub width : Size,
  pub height : Size,

  pub direction : LayoutDirection,
  pub alignment_horizonal : LayoutAlignment,
  pub alignment_vertical : LayoutAlignment, 
}


