
/*
About the layout problem 

2D Layout ht(horizontal) and vt(vertical)

Dynamical (Max, Min)
Relative (Relative)
Fixed (Pixel, Content)


# Instead of pixel make layoutsize (also effects font, those two should be linked) LayoutSized 1 Unit := FontHeight

Wihout padding ore margin considerations 


*/

use crate::objects::Quad2D;


#[derive(PartialEq, Clone, Copy)]
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


#[derive(PartialEq, Clone, Copy)]
pub struct ComputedLayout {
  pub width : u32,
  pub height : u32,

  pub x: u32,
  pub y: u32
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
          computed: ComputedLayout { width: 0, height: 0, x: 0, y: 0 },
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
    if layout.calculate(LayoutInfo { width, height, unit_size }).is_ok() {
      layout.position(0, 0);
    }
  }

  fn iter(&self) -> LayoutIter {
    self.layout().iter()
  }
}






pub enum LayoutError {
  InsufficientSpace,
  UnresolvedSize,
  MultipleMaxChildren,

  DoubleSameSized,
}




#[derive(Clone, Copy)]
struct LayoutInfo {
  width: u32,
  height: u32,
  unit_size: u32
}

impl LayoutInfo {

  pub fn shrink(&self, width: u32, height: u32) -> Self {
    LayoutInfo { width: width, height: height, unit_size: self.unit_size }
  }
}

pub struct LayoutIter<'a> {
  stack: Vec<&'a Layout>
}

impl<'a> Iterator for LayoutIter<'a> {
  type Item = &'a Layout;

  fn next(&mut self) -> Option<Self::Item> {
      if let Some(layout) = self.stack.pop() {
          for child in layout.layouts().collect::<Vec<_>>().into_iter().rev() {
              self.stack.push(child);
          }
          Some(&layout)
      } else {
          None
      }
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

  pub fn height(&self, height : u32, unit_size : u32) -> Result<u32, LayoutError>{

    match self.vertical() {
      Size::Relative(factor) => Ok((factor * height as f32) as u32),
      Size::Unit(factor) => Ok((factor *  unit_size as f32) as u32),
      Size::Content => Ok(height),
      Size::Max => Ok(height),
      Size::Same => return Err(LayoutError::DoubleSameSized),
    }
  } 


  pub fn calculate(&mut self, info : LayoutInfo) -> Result<(), LayoutError> {
    println!("{}, {}", info.width, info.height);
    match self.axis {
      Axis::Horizontal => self.calculate_horizontal(info)?,
      Axis::Vertical => self.calculate_vertical(info)?
    }
    println!("{}, {}", self.computed.width, self.computed.height);
    Ok(())
  }

  fn calculate_horizontal(&mut self, info : LayoutInfo) -> Result<(), LayoutError>{

    let width = self.width(info.width, info.unit_size)?;
    let height = self.height(info.height, info.unit_size)?;

    let mut child_width = width;
    
    // 1. First iterate over all fixed sized children
    for child in self.layouts_mut() {
      if !matches!(child.horizontal(), Size::Relative(_) | Size::Unit(_)) { continue }
      println!("info {}, {}", info.width, info.height);
      child.calculate(info)?;
      child_width -= child.computed.width;
    }
      
    for child in self.layouts_mut()  {
      if !matches!(child.horizontal(), Size::Content) { continue }

      child.calculate(info.shrink(child_width, height))?;
      child_width -= child.computed.width;
    }

    if child_width <= 0 { return Err(LayoutError::InsufficientSpace)}

    if let Some(child) = self.layouts_mut().find(|ch| ch.horizontal() == Size::Max) {
      child.calculate(info.shrink(child_width, height))?;
    }

    let width = if self.horizontal() == Size::Content {
      self.layouts().map(|l| l.computed.width).sum() 
    } else { 
      width 
    };

    self.computed.width = width;
    self.computed.height = if self.vertical() == Size::Content {
      self.layouts().map(|l| l.computed.height).max().unwrap_or(0)
    } else {
      height
    };

    println!("{}, {}", self.computed.height, self.computed.width);

    Ok(())
  }

  fn calculate_vertical(&mut self, info : LayoutInfo) -> Result<(), LayoutError>{

    let width = self.width(info.width, info.unit_size)?;
    let height = self.height(info.height, info.unit_size)?;

    let mut child_height = height;
    
    // 1. First iterate over all fixed sized children
    for child in self.layouts_mut() {
     if !matches!(child.vertical(), Size::Relative(_) | Size::Unit(_)) { continue }
     
      child.calculate(info.clone())?;
      child_height -= child.computed.height;
    }
      
    for child in self.layouts_mut(){
      if !matches!(child.vertical(), Size::Content) { continue }

      child.calculate(info.shrink(width, child_height))?;
      child_height -= child.computed.height;
    }

    if child_height <= 0 { return Err(LayoutError::InsufficientSpace)}

    if let Some(child) = self.layouts_mut().find(|ch| ch.vertical() == Size::Max) {
      child.calculate(info.shrink(width, child_height))?;
    }

    let height = if self.vertical() == Size::Content {
      self.layouts().map(|l| l.computed.height).sum() 
    } else { 
      height 
    };


    self.computed.height = height;

    self.computed.width = if self.horizontal() == Size::Content {
      self.layouts().map(|l| l.computed.width).max().unwrap_or(0)
    } else {
      width
    };

    Ok(())
  }

  pub fn position(&mut self, x: u32, y: u32) {
    match self.axis {
        Axis::Horizontal => self.position_horizontal(x, y),
        Axis::Vertical => self.position_vertical(x, y),
    }
  }

  pub fn position_horizontal(&mut self, x : u32, y: u32) {

    self.computed.x = x;
    self.computed.y = y;

    if self.children.len() == 0 { return; }

    let total_child_width = self.layouts().map(|ch| ch.computed.width).sum::<u32>();

    let halignment = self.halign;
    let valignment = self.valign;
    
    let start = self.computed.x;
    let width = self.computed.width;
    let space = self.computed.width - total_child_width;
    let evenly_spaced = space / (self.children.len() as u32 + 1) as u32;
    let mut running_width = 0;

    let ystart = self.computed.y;
    let yend = self.computed.y + self.computed.height;

    let largets_height = self.layouts().map(|ch| ch.computed.height).max().unwrap();

    for child in self.layouts_mut() {
      let x = match halignment {
        Alignment::Start => { 
          running_width += child.computed.width;
          start + running_width - child.computed.width
        }
        Alignment::End => {
          running_width += child.computed.width;
          start + width - running_width
          
        }
        Alignment::Center => {
          running_width += child.computed.width;
          start + (space / 2 as u32) + running_width - child.computed.width
        },
        Alignment::Even => {
          running_width += child.computed.width + evenly_spaced;
          start + running_width - child.computed.width
        }
      };

      let y = match valignment {
        Alignment::Start => ystart,
        Alignment::End => yend - child.computed.height,
        Alignment::Center => ystart + (yend - ystart - child.computed.height) / 2,
        Alignment::Even => ystart + (yend - ystart - largets_height) / 2,
      };

      child.position(x, y);
    }
  }


  pub fn position_vertical(&mut self, x : u32, y: u32) {

    self.computed.x = x;
    self.computed.y = y;

    if self.children.len() == 0 { return; }

    let total_child_height = self.layouts().map(|ch| ch.computed.height).sum::<u32>();
    let largest_width = self.layouts().map(|ch| ch.computed.width).max().unwrap();

    let halignment = self.halign;
    let valignment = self.valign;
    
    let start = self.computed.y;
    let height = self.computed.height;

    let space = height - total_child_height;
    let evenly_spaced = space / (self.children.len() as u32 + 1) as u32;
    let mut running_height = 0;

    let xstart = self.computed.x;
    let xend = self.computed.x + self.computed.width;


    for child in self.layouts_mut() {
      let y = match valignment {
        Alignment::Start => { 
          running_height += child.computed.height;
          start + running_height - child.computed.height
        }
        Alignment::End => {
          running_height += child.computed.height;
          start + height - running_height
          
        }
        Alignment::Center => {
          running_height += child.computed.height;
          start + (space / 2 as u32) + running_height - child.computed.height
        },
        Alignment::Even => {
          running_height += child.computed.height + evenly_spaced;
          start + running_height - child.computed.height
        }
      };

      let x = match halignment {
        Alignment::Start => xstart,
        Alignment::End => xend - child.computed.width,
        Alignment::Center => xstart + (xend - xstart - child.computed.width) / 2,
        Alignment::Even => xstart + (xend - xstart - largest_width) / 2,
      };

      child.position(x, y);
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
      assert_eq!(layout.computed.width, 600); // 10*10 + 20*10 + 30*10
      assert_eq!(layout.computed.height, 150); // max height 15*10
      
      // Check child positions
      let children: Vec<_> = layout.layouts().collect();
      assert_eq!(children[0].computed.x, 0);
      assert_eq!(children[1].computed.x, 100);
      assert_eq!(children[2].computed.x, 300);
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
      assert_eq!(layout.computed.height, 450); // 5*10 + 10*10 + 30*10
      assert_eq!(layout.computed.width, 200); // max width 20*10
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
      assert_eq!(children[1].computed.width, 600);
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
      assert_eq!(layout.computed.width, 1000); // 100*10
      assert_eq!(layout.computed.height, 1000); // 100*10
      
      // First child should be a horizontal layout with two elements
      let children: Vec<_> = layout.layouts().collect();
      assert_eq!(children[0].computed.height, 200); // 20*10
      
      // And it should have two children
      let grandchildren: Vec<_> = children[0].layouts().collect();
      assert_eq!(grandchildren.len(), 2);
      assert_eq!(grandchildren[0].computed.width, 100); // 10*10
      assert_eq!(grandchildren[1].computed.width, 200); // 20*10
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
      assert_eq!(children[0].computed.x, 450); // (1000 - 100) / 2
      assert_eq!(children[0].computed.y, 450); // (1000 - 100) / 2
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
      assert_eq!(layout.computed.width, 400); // 10*10 + 30*10
      assert_eq!(layout.computed.height, 200); // max(20*10, 10*10)
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
      
      assert_eq!(children[0].computed.width, 300); // 30% of 1000
      assert_eq!(children[0].computed.height, 500); // 50% of 1000
      assert_eq!(children[1].computed.width, 700); // 70% of 1000
      assert_eq!(children[1].computed.height, 800); // 80% of 1000
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
      assert_eq!(children[0].computed.width, 300);
      assert_eq!(children[0].computed.height, 300);
  }
  
  #[test]
  fn test_empty_layout() {
      let mut root = TestElement::new()
          .width(Size::Unit(50.0))
          .height(Size::Unit(50.0));
          
      root.calculate(1000, 1000, 10);
      
      let layout = root.layout();
      assert_eq!(layout.computed.width, 500);
      assert_eq!(layout.computed.height, 500);
  }
  
}