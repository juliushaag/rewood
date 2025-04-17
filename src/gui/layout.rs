
/*
About the layout problem 

2D Layout ht(horizontal) and vt(vertical)

Dynamical (Max, Min)
Relative (Relative)
Fixed (Pixel, Content)


# Instead of pixel make layoutsize (also effects font, those two should be linked) LayoutSized 1 Unit := FontHeight

Without padding ore margin considerations 

Okay so how do we work on overflow components
- Outer frame (outer frame contraint) 
- Inner frame ()

*/

use std::cmp::max;

use crate::objects::Quad2D;


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Size {

  Max, // Take up all the remaing space (if there is any)
  Relative(f32),    // Relative percentage, compared to the parent frame
  Unit(f32),        // Multiple of font unit, should be used primarily to style fixed elements 

  Content,  // comes out to the maximum sized child (has always a fixed size (has to evaluate to unit)  
  Same,     // width adjusts to the same as height, or the other way aroud (has to evaluate to unit)
}


#[derive(PartialEq, Clone, Copy)]
pub enum Axis {
  Horizontal,
  Vertical
}


#[derive(PartialEq, Clone, Copy)]
pub enum Alignment {
  Start,
  End,
  Center,
  Even,
}


#[derive(PartialEq, Clone, Copy, Default)]
pub struct ComputedLayout {
  pub outer_dim : (u32, u32),
  pub outer_pos : (u32, u32),
  
  pub inner_dim : (u32, u32),
  pub inner_pos : (u32, u32),

  pub content_dim : (u32, u32),
  pub content_pos : (u32, u32),

  pub core_dim : (u32, u32),
  pub core_pos : (u32, u32),
}

pub struct Layout {
  margin: [f32; 4],
  padding: [f32; 4],
  border: [f32; 4],

  axis: Axis,
  
  hsize: Size,
  vsize: Size,

  halign: Alignment,
  valign: Alignment,

  pub computed : ComputedLayout,

  children: Vec<Box<dyn LayoutElement>>,
}


impl Default for Layout {
    fn default() -> Self {
        Self { 
          margin: Default::default(), 
          padding: Default::default(), 
          border: Default::default(), 
          axis: Axis::Horizontal, 
          hsize: Size::Content, 
          vsize: Size::Content, 
          halign: Alignment::Start, 
          valign: Alignment::Start, 
          computed: Default::default(),
          children: Default::default() 
      }
    }
}



pub trait LayoutElement {
  fn layout(&self) -> &Layout;
  fn layout_mut(&mut self) -> &mut Layout;

  fn boxed(self) -> Box<dyn LayoutElement> where Self : Sized + 'static {
    Box::new(self)
  }

  fn vertical(mut self, children: impl IntoIterator<Item = Box<dyn LayoutElement>>) -> Self
  where
    Self: Sized,
  {
    let layout = self.layout_mut();
    layout.axis = Axis::Vertical;
    layout.children.extend(children);
    self
  }

  fn horizontal(mut self, children: impl IntoIterator<Item = Box<dyn LayoutElement>>) -> Self
  where
    Self: Sized,
  {
    let layout = self.layout_mut();
    layout.axis = Axis::Horizontal;
    layout.children.extend(children);
    self
  }

  fn align(mut self, align : Alignment) -> Self
  where
    Self: Sized,
  {
    let axis = self.layout().axis;
    match axis {
        Axis::Horizontal => self.layout_mut().halign = align,
        Axis::Vertical => self.layout_mut().valign = align,
    }
    self
  }

  
  fn margin(mut self, margin: f32) -> Self
  where
    Self: Sized, 
  {
    self.layout_mut().margin.iter_mut().for_each(|x| { *x = margin }); 
    self
  }

  fn margin_axis(mut self, xmargin: f32, ymargin : f32) -> Self
  where
    Self: Sized, 
  {
    let layout = self.layout_mut();
    layout.margin[0] = xmargin;
    layout.margin[1] = xmargin;
    layout.margin[2] = ymargin;
    layout.margin[3] = ymargin;   
    self
  }

  fn margin_all(mut self, left: f32, right : f32, top : f32, bottom : f32) -> Self
  where
    Self: Sized, 
  {
    let layout = self.layout_mut();
    layout.margin[0] = left;
    layout.margin[1] = right;
    layout.margin[2] = top;
    layout.margin[3] = bottom;   
    self
  }
  
  
  fn padding(mut self, padding: f32) -> Self
  where
    Self: Sized, 
  {
    self.layout_mut().padding.iter_mut().for_each(|x| { *x = padding }); 
    self
  }

  fn padding_axis(mut self, xpadding: f32, ypadding : f32) -> Self
  where
    Self: Sized, 
  {
    let layout = self.layout_mut();
    layout.padding[0] = xpadding;
    layout.padding[1] = xpadding;
    layout.padding[2] = ypadding;
    layout.padding[3] = ypadding;   
    self
  }

  fn padding_all(mut self, left: f32, right : f32, top : f32, bottom : f32) -> Self
  where
    Self: Sized, 
  {
    let layout = self.layout_mut();
    layout.padding[0] = left;
    layout.padding[1] = right;
    layout.padding[2] = top;
    layout.padding[3] = bottom;   
    self
  }
  

  fn width(mut self, sizing: Size) -> Self
  where
    Self: Sized,
  {
    self.layout_mut().hsize = sizing;
    self
  }

  fn height(mut self, sizing: Size) -> Self
  where
    Self: Sized,
  {
    self.layout_mut().vsize = sizing;
    self
  }

  fn calculate(&mut self, width: u32, height: u32, unit_size: u32) {
    let layout = self.layout_mut();
    let layout_info = LayoutInfo { width, height, x: 0, y: 0, unit_size };
    if layout.calculate(layout_info).is_ok() {
      layout.position(layout_info);
    }
  }

  fn iter(&self) -> LayoutIter {
    self.layout().iter()
  }
}


pub enum LayoutError {
  MultipleMaxChildren,
  DoubleSameSized,
}




#[derive(Clone, Copy)]
struct LayoutInfo {
  width: u32,
  height: u32,
  x: u32,
  y: u32,
  unit_size: u32
}

impl LayoutInfo {

  pub fn shrink(&self, width: u32, height: u32) -> Self {
    LayoutInfo { width: width, height: height, x: self.x, y: self.y, unit_size: self.unit_size }
  }

  pub fn shrink_frame(&self, x: u32, y: u32, width: u32, height: u32) -> Self {
    LayoutInfo { width: width, height: height, x: x, y: y, unit_size: self.unit_size }
  }
}

pub struct LayoutIter<'a> {
  stack: Vec<&'a Layout>
}

impl<'a> Iterator for LayoutIter<'a> {
  type Item = &'a Layout;

  fn next(&mut self) -> Option<Self::Item> {
    let layout = self.stack.pop()?;
    self.stack.extend(layout.layouts());
    Some(layout)
  }
}


impl Layout {

  pub fn iter(&self) -> LayoutIter {
    LayoutIter { stack: vec![self] }
  }

  pub fn layouts(&self) -> impl Iterator<Item = &Layout> {
    self.children.iter().map(|ch| ch.layout())
  }

  pub fn layouts_mut(&mut self) -> impl Iterator<Item = &mut Layout> {
    self.children.iter_mut().map(|ch| ch.layout_mut())
  }

  pub fn horizontal(&self) -> Size {
    if self.hsize != Size::Same { self.hsize } else {self.vsize }
  }

  pub fn vertical(&self) -> Size {
    if self.vsize != Size::Same { self.vsize } else { self.hsize } 
  }

  pub fn width(&self, width : u32, unit_size : u32) -> Result<u32, LayoutError>{
    match self.horizontal() {
      Size::Relative(factor) => Ok((factor * width as f32 ) as u32),
      Size::Unit(factor) => Ok((factor *  unit_size as f32) as u32),
      Size::Content => Ok(width),
      Size::Max => Ok(width),
      Size::Same => return Err(LayoutError::DoubleSameSized),
    }
  }

  pub fn inner_width(&self, width : u32, unit_size : u32) -> Result<u32, LayoutError>{

   let mut width = self.width(width, unit_size);

    if let Ok(width) = &mut width  {  
      *width = width.saturating_sub(((self.margin[0] + self.margin[1]) * unit_size as f32)  as u32);
    }

    width
  } 

  pub fn content_width(&self, width : u32, unit_size : u32) -> Result<u32, LayoutError> {
    let mut width = self.inner_width(width, unit_size);

    if let Ok(width) = &mut width  {  
      *width = width.saturating_sub(((self.padding[0] + self.padding[1]) * unit_size as f32)  as u32);
    }

    width
  }


  pub fn height(&self, height : u32, unit_size : u32) -> Result<u32, LayoutError> {
    match self.vertical() {
      Size::Relative(factor) => Ok((factor * height as f32) as u32),
      Size::Unit(factor) => Ok((factor *  unit_size as f32) as u32),
      Size::Content => Ok(height),
      Size::Max => Ok(height),
      Size::Same => return Err(LayoutError::DoubleSameSized),
    }
  }

  pub fn inner_height(&self, height : u32, unit_size : u32) -> Result<u32, LayoutError>{

    let mut height = self.height(height, unit_size);

    if let Ok(height) = &mut height {
      *height = height.saturating_sub(((self.margin[2] + self.margin[3]) * unit_size as f32)  as u32);
    }
    height
  } 

  pub fn content_height(&self, height : u32, unit_size : u32) -> Result<u32, LayoutError>{

    let mut height = self.inner_height(height, unit_size);

    if let Ok(height) = &mut height {
      *height = height.saturating_sub(((self.padding[2] + self.padding[3]) * unit_size as f32)  as u32);
    }
    height
  } 


  pub fn calculate(&mut self, info : LayoutInfo) -> Result<(), LayoutError> {

    self.computed.outer_dim = (self.width(info.width, info.unit_size)?, self.height(info.height, info.unit_size)?);
    self.computed.inner_dim = (self.inner_width(info.width, info.unit_size)?, self.inner_height(info.height, info.unit_size)?);
    self.computed.content_dim = (self.content_width(info.width, info.unit_size)?, self.content_height(info.height, info.unit_size)?);

    let (child_width, child_height) = match self.axis {
      Axis::Horizontal => {
        self.calculate_horizontal(info.shrink(self.computed.content_dim.0, self.computed.content_dim.1))?;
        let child_width : u32 = self.layouts().map(|l| l.computed.outer_dim.0).sum();
        let child_height = self.layouts().map(|l| l.computed.outer_dim.1).max().unwrap_or(0);
        (child_width, child_height)
      }
      Axis::Vertical => {
        self.calculate_vertical(info.shrink(self.computed.content_dim.0, self.computed.content_dim.1))?;
        let child_width : u32 = self.layouts().map(|l| l.computed.outer_dim.0).max().unwrap_or(0);
        let child_height = self.layouts().map(|l| l.computed.outer_dim.1).sum();
        (child_width, child_height)
      }
    };

    // 5) compute own size
    self.computed.core_dim = (child_width, child_height);
    if self.horizontal() == Size::Content && child_width < self.computed.content_dim.0 {
      self.computed.content_dim.0 = child_width;
      self.computed.inner_dim.0 = child_width + ((self.padding[0] + self.padding[1]) * info.unit_size as f32) as u32;
      self.computed.outer_dim.0 = child_width + ((self.margin[0] + self.margin[1]) * info.unit_size as f32) as u32;
    }

    if self.vertical() == Size::Content && child_height < self.computed.content_dim.1 {
      self.computed.content_dim.1 = child_height;
      self.computed.inner_dim.1 = child_height + ((self.padding[2] + self.padding[3]) * info.unit_size as f32) as u32;
      self.computed.outer_dim.1 = child_height + ((self.margin[2] + self.margin[3]) * info.unit_size as f32) as u32;
    }

    Ok(())
  }
  fn calculate_horizontal(&mut self, info: LayoutInfo) -> Result<(), LayoutError> {

    let width =self.computed.content_dim.0; 
    let height = self.computed.content_dim.1;

    // 0) group children by sizing mode
    let mut rem : i32 = width as i32;
    let mut fixed = Vec::new();
    let mut content = Vec::new();
    let mut max_child = None;

    for child in self.layouts_mut() {
      match child.horizontal() {
        Size::Relative(_) | Size::Unit(_) => fixed.push(child),
        Size::Content           => content.push(child),
        Size::Max               => {
          if max_child.is_some() { 
            return Err(LayoutError::MultipleMaxChildren);
          };
          max_child = Some(child)
        }
        _ => {}
      }
    }
    // Combine fixed and content layouts
    fixed.append(&mut content);
    
    // 1) layout fixed and content-size
    for ch in &mut fixed {
      ch.calculate(info)?;
      rem = rem - ch.computed.outer_dim.0 as i32;
    }

    // 2) remaining max-sized child
    if let Some(ch) = max_child {
      ch.calculate(info.shrink(max(rem, 0) as u32, height))?;
    }    

    Ok(())
  }

  fn calculate_vertical(&mut self, info: LayoutInfo) -> Result<(), LayoutError> {
    

    let width = self.computed.content_dim.0;
    let height = self.computed.content_dim.1;
    let mut rem : i32 = height as i32;

    // 0) group children by sizing
    let mut fixed = Vec::new();
    let mut content = Vec::new();
    let mut max_child = None;
    
    for child in self.layouts_mut() {
      match child.vertical() {
        Size::Relative(_) | Size::Unit(_) => fixed.push(child),
        Size::Content               => content.push(child),
        Size::Max => {
          if max_child.is_some() {
            return Err(LayoutError::MultipleMaxChildren);
          }
          max_child = Some(child);
        }
        _ => {}
      }
    }
    
    // Combine fixed and content layouts
    fixed.append(&mut content);
    
    // 1) layout fixed and content-size
    for ch in &mut fixed {
      ch.calculate(info)?;
      rem = rem - ch.computed.outer_dim.1 as i32;
    }

    // 3) remaining max-sized child
    if let Some(ch) = max_child {
      ch.calculate(info.shrink(width, max(rem, 0) as u32))?;
    }    

    Ok(())
  }

  pub fn position(&mut self, info : LayoutInfo) {
    self.computed.outer_pos.0 = info.x;
    self.computed.outer_pos.1 = info.y;
 
    self.computed.inner_pos.0 = self.computed.outer_pos.0 + (self.margin[0] * info.unit_size as f32) as u32; 
    self.computed.inner_pos.1 = self.computed.outer_pos.1 + (self.margin[2] * info.unit_size as f32) as u32; 


    self.computed.content_pos.0 = self.computed.inner_pos.0 + (self.padding[0] * info.unit_size as f32) as u32; 
    self.computed.content_pos.1 = self.computed.inner_pos.1 + (self.padding[2] * info.unit_size as f32) as u32; 

    if self.children.len() == 0 { return; }

    let layout_info = info.shrink_frame(
      self.computed.content_pos.0, 
      self.computed.content_pos.1, 
      max(self.computed.content_dim.0, self.computed.core_dim.0), 
      max(self.computed.content_dim.1, self.computed.core_dim.1)
    );
    

    match self.axis {
      Axis::Horizontal => self.position_horizontal(layout_info),
      Axis::Vertical => self.position_vertical(layout_info),
    }
  }

  pub fn position_horizontal(&mut self, info : LayoutInfo) {

    let total_child_width = self.computed.core_dim.0;
    let largets_height = self.computed.core_dim.1;

    let halignment = self.halign;
    let valignment = self.valign;
    
    let start = info.x;
    let width = info.width;

    let space = width - total_child_width;
    let evenly_spaced = space / (self.children.len() as u32 + 1) as u32;
    let mut running_width = 0;

    let ystart = info.y;
    let yend = info.y + info.height;


    for child in self.layouts_mut() {
      let x = match halignment {
        Alignment::Start => { 
          running_width += child.computed.outer_dim.0;
          start + running_width - child.computed.outer_dim.0
        }
        Alignment::End => {
          running_width += child.computed.outer_dim.0;
          start + width - running_width
          
        }
        Alignment::Center => {
          running_width += child.computed.outer_dim.0;
          start + (space / 2 as u32) + running_width - child.computed.outer_dim.0
        },
        Alignment::Even => {
          running_width += child.computed.outer_dim.0 + evenly_spaced;
          start + running_width - child.computed.outer_dim.0
        }
      };

      let y = match valignment {
        Alignment::Start => ystart,
        Alignment::End => yend - child.computed.outer_dim.1,
        Alignment::Center => ystart + (yend - ystart - child.computed.outer_dim.1) / 2,
        Alignment::Even => ystart + (yend - ystart - largets_height) / 2,
      };

      child.position(info.shrink_frame(x, y, child.computed.outer_dim.0, child.computed.outer_dim.1));
    }
  }


  pub fn position_vertical(&mut self, info : LayoutInfo) {

    let total_child_height = self.computed.core_dim.1;
    let largest_width = self.computed.core_dim.0;
    
    let halignment = self.halign;
    let valignment = self.valign;
    
    let start = info.y;
    let height = info.height;

    let space = height - total_child_height;
    let evenly_spaced = space / (self.children.len() as u32 + 1) as u32;
    let mut running_height = 0;

    let xstart = info.x;
    let xend = info.x + info.width;


    for child in self.layouts_mut() {
      let y = match valignment {
        Alignment::Start => { 
          running_height += child.computed.outer_dim.1;
          start + running_height - child.computed.outer_dim.1
        }
        Alignment::End => {
          running_height += child.computed.outer_dim.1;
          start + height - running_height
          
        }
        Alignment::Center => {
          running_height += child.computed.outer_dim.1;
          start + (space / 2 as u32) + running_height - child.computed.outer_dim.1
        },
        Alignment::Even => {
          running_height += child.computed.outer_dim.1 + evenly_spaced;
          start + running_height - child.computed.outer_dim.1
        }
      };

      let x = match halignment {
        Alignment::Start => xstart,
        Alignment::End => xend - child.computed.outer_dim.0,
        Alignment::Center => xstart + (xend - xstart - child.computed.outer_dim.0) / 2,
        Alignment::Even => xstart + (xend - xstart - largest_width) / 2,
      };

      child.position(info.shrink_frame(x, y, child.computed.outer_dim.0, child.computed.outer_dim.1));
    }
  }

}


#[cfg(test)]
mod tests {
  use super::*;

  struct TestElement {
    layout: Layout,
  }

  impl TestElement {
    fn new() -> Self {
      Self { layout: Layout::default() }
    }
    
    // Helper to create a fixed-size element
    fn with_size(width: f32, height: f32) -> Self {
      let mut elem = Self::new();
      elem.layout_mut().hsize = Size::Unit(width);
      elem.layout_mut().vsize = Size::Unit(height);
      elem
    }
  }

  impl LayoutElement for TestElement {
    fn layout(&self) -> &Layout {
      &self.layout
    }
    
    fn layout_mut(&mut self) -> &mut Layout {
      &mut self.layout
    }
  }
   
  #[test]
  fn test_horizontal_basic() {
      let mut root = TestElement::new()
          .horizontal(vec![
              TestElement::with_size(10.0, 5.0).boxed(),
              TestElement::with_size(20.0, 10.0).boxed(),
              TestElement::with_size(30.0, 15.0).boxed(),
          ]);
          
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      assert_eq!(layout.computed.outer_dim.0, 600); // 10*10 + 20*10 + 30*10
      assert_eq!(layout.computed.outer_dim.1, 150); // max height 15*10
      
      // Check child positions
      let children: Vec<_> = layout.layouts().collect();
      assert_eq!(children[0].computed.outer_pos.0, 0);
      assert_eq!(children[1].computed.outer_pos.0, 100);
      assert_eq!(children[2].computed.outer_pos.0, 300);
  }
  
  #[test]
  fn test_vertical_basic() {
      let mut root = TestElement::new()
          .vertical(vec![
              TestElement::with_size(10.0, 5.0).boxed(),
              TestElement::with_size(20.0, 10.0).boxed(),
              TestElement::with_size(15.0, 30.0).boxed(),
          ]);
          
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      assert_eq!(layout.computed.outer_dim.1, 450); // 5*10 + 10*10 + 30*10
      assert_eq!(layout.computed.outer_dim.0, 200); // max width 20*10
  }
  
  #[test]
  fn test_max_sized_child() {
      let mut root = TestElement::new()
          .horizontal(vec![
              TestElement::with_size(10.0, 5.0).boxed(),
              TestElement::new().width(Size::Max).height(Size::Unit(10.0)).boxed(),
              TestElement::with_size(30.0, 15.0).boxed(),
          ]);
          
      root.calculate(1000, 100, 10);
      
      let layout = root.layout();
      let children: Vec<_> = layout.layouts().collect();
      
      // Max-sized child should take remaining space (1000 - 100 - 300 = 600)
      assert_eq!(children[1].computed.outer_dim.0, 600);
  }
  
  #[test]
  fn test_nested_layouts() {
      let mut root = TestElement::new()
          .width(Size::Unit(100.0))
          .height(Size::Unit(100.0))
          .vertical(vec![
              TestElement::new()
                  .height(Size::Unit(20.0))
                  .horizontal(vec![
                      TestElement::with_size(10.0, 5.0).boxed(),
                      TestElement::with_size(20.0, 5.0).boxed(),
                  ])
                  .boxed(),
              TestElement::with_size(50.0, 30.0).boxed(),
          ]);
          
      root.calculate(2000, 2000, 10);
      
      let layout = root.layout();
      assert_eq!(layout.computed.outer_dim.0, 1000); // 100*10
      assert_eq!(layout.computed.outer_dim.1, 1000); // 100*10
      
      // First child should be a horizontal layout with two elements
      let children: Vec<_> = layout.layouts().collect();
      assert_eq!(children[0].computed.outer_dim.1, 200); // 20*10
      
      // And it should have two children
      let grandchildren: Vec<_> = children[0].layouts().collect();
      assert_eq!(grandchildren.len(), 2);
      assert_eq!(grandchildren[0].computed.outer_dim.0, 100); // 10*10
      assert_eq!(grandchildren[1].computed.outer_dim.0, 200); // 20*10
  }
  
  #[test]
  fn test_alignment_center() {
      let mut elem = TestElement::new()
        .width(Size::Max)
        .height(Size::Max);
      
      elem.layout_mut().halign = Alignment::Center;
      elem.layout_mut().valign = Alignment::Center;
      
      let mut root = elem.horizontal(vec![
          TestElement::with_size(10.0, 10.0).boxed(),
      ]);
      
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      let children: Vec<_> = layout.layouts().collect();
      
      // Child should be centered
      assert_eq!(children[0].computed.outer_pos.0, 450); // (1000 - 100) / 2
      assert_eq!(children[0].computed.outer_pos.1, 450); // (1000 - 100) / 2
  }
  
  #[test]
  fn test_content_sizing() {
      let mut root = TestElement::new()
          .width(Size::Content)
          .height(Size::Content)
          .horizontal(vec![
              TestElement::with_size(10.0, 20.0).boxed(),
              TestElement::with_size(30.0, 10.0).boxed(),
          ]);
          
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      assert_eq!(layout.computed.outer_dim.0, 400); // 10*10 + 30*10
      assert_eq!(layout.computed.outer_dim.1, 200); // max(20*10, 10*10)
  }
  
  #[test]
  fn test_relative_sizing() {
      let mut root = TestElement::new()
          .width(Size::Unit(100.0))
          .height(Size::Unit(100.0))
          .horizontal(vec![
              TestElement::new().width(Size::Relative(0.3)).height(Size::Relative(0.5)).boxed(),
              TestElement::new().width(Size::Relative(0.7)).height(Size::Relative(0.8)).boxed(),
          ]);
          
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      let children: Vec<_> = layout.layouts().collect();
      
      assert_eq!(children[0].computed.outer_dim.0, 300); // 30% of 1000
      assert_eq!(children[0].computed.outer_dim.1, 500); // 50% of 1000
      assert_eq!(children[1].computed.outer_dim.0, 700); // 70% of 1000
      assert_eq!(children[1].computed.outer_dim.1, 800); // 80% of 1000
  }
  
  #[test]
  fn test_same_size() {
      let mut root = TestElement::new()
          .width(Size::Unit(50.0))
          .height(Size::Unit(100.0))
          .horizontal(vec![
              TestElement::new().width(Size::Same).height(Size::Unit(30.0)).boxed(),
          ]);
          
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      let children: Vec<_> = layout.layouts().collect();
      
      // Width should match height (30*10 = 300)
      assert_eq!(children[0].computed.outer_dim.0, 300);
      assert_eq!(children[0].computed.outer_dim.1, 300);
  }
  
  #[test]
  fn test_empty_layout() {
      let mut root = TestElement::new()
          .width(Size::Unit(50.0))
          .height(Size::Unit(50.0));
          
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      assert_eq!(layout.computed.outer_dim.0, 500);
      assert_eq!(layout.computed.outer_dim.1, 500);
  }
}
