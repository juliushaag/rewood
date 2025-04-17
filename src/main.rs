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
                .margin(&[0.5, 0.5])
                .vertical(vec![
                    Container::new("Left Part")
                        .width(Size::Max)
                        .height(Size::Unit(20.0))
                        .margin(&[0.5, 0.5])
                        .boxed(),
                    Container::new("Left Part")
                        .width(Size::Max)
                        .height(Size::Max)
                        .margin(&[0.5, 0.5])
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
