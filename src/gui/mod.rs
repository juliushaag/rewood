pub mod container;
pub mod layout;

use layout::Layout;


pub trait GUIElement {

  fn children(&self) -> &Vec<Box<dyn GUIElement>>;

  fn calculate_layout_horizontal(&mut self, width: u32, height: u32) -> Result<Layout, LayoutError>;


  fn calculate_layout_vertical(&mut self, width: u32, height: u32) -> Result<Layout, LayoutError>;
}
