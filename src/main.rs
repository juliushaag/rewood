use winit::event_loop::EventLoop;
mod application;
mod user_event;

mod renderer;

use user_event::{ UserEvent };
use application::{ Application, Layer};

 
struct TestLayer {}

impl Layer for  TestLayer {
    fn new(app : &mut Application) -> Self {
        TestLayer {  }
    }

    fn on_event(&mut self, event : UserEvent) -> Result<(), String> {
        Ok(())
    }

    fn on_update(&mut self) -> Result<(), String> {
        Ok(())
    }
}





fn main() -> Result<(), String> {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    let mut app = Application::new(&event_loop);

    app.attach::<TestLayer>();
 

    let _ = event_loop.run(move |event, control_flow| app.event_handler(event, control_flow));

    Ok(())
}
