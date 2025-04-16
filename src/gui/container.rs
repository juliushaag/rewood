
use crate::gui::layout::{Layout, LayoutElement, };

pub struct Container {
  name : String,
  layout: Layout,
}


impl LayoutElement for Container {
  fn layout(&self) -> &Layout {
    &self.layout
  }

  fn layout_mut(&mut self) -> &mut Layout {
    &mut self.layout
  }
}

impl Container {

  pub fn new<T: Into<String>>(name : T) -> Self {
    Container { name: name.into(), layout: Default::default() }
  }
}






/*

let root = Container::new("Left Part")
  .height(Size::Max)
  .width(Size::Resizable(0.2))
  .resizable(Some(0.2), None, Resizable::Fold)
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