#[allow(dead_code)]
#[allow(unused_variables)]

mod application;
mod user_event;
mod renderer;
mod objects;

#[macro_use]
mod gui;

use gui::{container::Container, layout::{Alignment, LayoutElement, Size}};
use application::Application;


fn main() -> Result<(), String>{

    let mut app = Application::new()?;


    let layout = Container::new("Main Page")
        .horizontal(vec![
            Container::new("Left Part")
                .width(Size::Unit(20.0))
                .height(Size::Max)
                .align(Alignment::Even)
                .vertical(vec![
                    Container::new("Left Part")
                        .width(Size::Unit(10.0))
                        .height(Size::Same)
                        .boxed(),
                    Container::new("Left Part")
                        .width(Size::Unit(12.0))
                        .height(Size::Max)
                        .boxed(),
                ])
                .boxed(),
            Container::new("Right Part")
                .width(Size::Max)
                .height(Size::Max)
                .boxed(),
            Container::new("Left Part")
                .width(Size::Unit(20.0))
                .height(Size::Max)
                .boxed(),
        ]);




    app.show(layout);
    
    
    while app.running() { app.update(); }

    Ok(())
}
