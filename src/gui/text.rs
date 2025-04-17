use skia_safe::{Font, Paint};

use super::layout::{self, Layout, LayoutElement, Size};





pub struct TextElement {
  content : String,
  layout: Layout,

  paint : Paint,
  font : Font,
}


impl LayoutElement for TextElement {
  fn layout(&self) -> &Layout {
    &self.layout
  }

  fn layout_mut(&mut self) -> &mut Layout {
    &mut self.layout
  }
}

impl TextElement {

  pub fn new<T: Into<String>>(content : T) -> Self {

    TextElement { content: content.into(), layout: Default::default(), paint: Paint::default(), font: Font::default() }

  }

  fn update_unit_size(&mut self, unit_size : u32) {
    let (length , rect) = self.font.measure_str(&self.content, Some(&self.paint));

    self.layout.hsize = Size::Unit(rect.width() / unit_size as f32);
    self.layout.vsize = Size::Unit(rect.height() / unit_size as f32);  
  }
}




