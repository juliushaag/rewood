#[allow(dead_code)]
#[allow(unused_variables)]

mod application;
mod user_event;
mod objects;

#[macro_use]
mod gui;

use gui::{container::Container, layout::{Alignment, LayoutElement, Size}};
use application::Application;


fn main() -> Result<(), String>{



    let layout = Container::new("Main Page")
        .horizontal(vec![
            Container::new("Left Part")
                .width(Size::Unit(20.0))
                .height(Size::Max)
                .margin(0.5)
                .padding(1.0)
                .vertical(vec![
                    Container::new("Left Part")
                        .width(Size::Max)
                        .height(Size::Unit(20.0))
                        .boxed(),
                    Container::new("Left Part")
                        .width(Size::Max)
                        .height(Size::Unit(20.0))
                        .margin(0.7)
                        .boxed(),
                ])

                .align(Alignment::Even)
                .boxed(),
            Container::new("Right Part")
                .width(Size::Max)
                .height(Size::Max)
                .horizontal(vec![
                    Container::new("Left Part")
                    .width(Size::Same)
                    .height(Size::Unit(20.0))
                    .margin(0.7)
                    .boxed(),
                    Container::new("Left Part")
                    .width(Size::Same)
                    .height(Size::Unit(20.0))
                    .margin(0.7)
                    .boxed(),
                    Container::new("Left Part")
                    .width(Size::Same)
                    .height(Size::Unit(40.0))
                    .margin(0.7)
                    .boxed(),
                    Container::new("Left Part")
                    .width(Size::Same)
                    .height(Size::Unit(20.0))
                    .margin(0.7)
                    .boxed(),    
                ])
                .align(Alignment::Even)
                .boxed(),
            Container::new("Left Part")
                .width(Size::Unit(20.0))
                .height(Size::Max)
                .boxed()
        ]);

    
    let app = Application::new(layout);
    app.run();

    Ok(())
}
