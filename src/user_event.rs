


#[derive(PartialEq)]
pub enum UserEvent {
  Quit,
  Click(f32, f32),
  Resize(u32, u32),
  MouseMoved(f32, f32),
  None,
}