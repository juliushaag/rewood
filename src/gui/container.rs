use wgpu::naga::MathFunction;

use super::{ Layout, GUIElement, LayoutError, Size };

struct Container {
  width_size : Size, 
  children: Vec<Box<dyn GUIElement>>
}


impl GUIElement for Container {
    fn children(&self) -> &Vec<Box<dyn GUIElement>> {
      return &self.children
    }

   
  fn calculate_layout_horizontal(&mut self, width: u32, height: u32) -> Result<Layout, LayoutError> {
    match  {
        
    }
  }


  fn calculate_layout_vertical(&mut self, width: u32, height: u32) -> Result<Layout, LayoutError> {

  }
}





/*

let root = Container::new("Left Part")
  .height(Size::Max)
  .width(Size::Resizable(0.2, RESIZE::FOLD))
  .vertical({
    Container::new("Connection Status")
      .width(Size::Max)
      .height(Size::Max)
      .vertical({
        Container::new("Connection Indicator")
          .rounded(Size::Percent(0.5))
          .height(Size::Pixel(10px))
          .width(Size::Same)
          .align(Alingment::Center, Alingment::Center)
          .on_signal::<ConnectionSignal>(|signal, container| {  
            match signal {
              ConnectionSignal::Disconnected => { container.color(Color::Red); },
              ConnectionSignal::Connected => { container.color(Color::Green);},
            }
          })
      })
  })

app.display(root);

app.run::<ConnectionHandler>();

app.send_signal(ConnectionSignal::Disconnected);

app.run();
*/