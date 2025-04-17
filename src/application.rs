use rand::Rng;

use crate::{gui::layout::{self, LayoutElement}, objects::{Color, Quad2D}, user_event::UserEvent};

use super::renderer::{ Context, Renderer, Window };



pub struct Application {
    window : Window,
    renderer : Renderer,

    context : Context,
    running : bool,

    layout : Option<Box<dyn LayoutElement>>
}




impl Application {


    pub fn new() -> Result<Self, String> {

        let mut context = Context::new()?;

        
        let mut window = context.create_window("Redwood", 800, 600);

        let renderer = context.create_renderer(&mut window);

        Ok(Application { window, renderer, context, running: true, layout: None})
    }

    pub fn update(&mut self) {
        while let Some(event) = self.poll() {
            match event {
                UserEvent::Quit => self.running = false,
                UserEvent::Resize(width, height) => {
                    self.renderer.resize(width, height);
                    self.render();
                }
                _ => continue
        } 

        self.render();
    }
}

    pub fn render(&mut self) {
        self.renderer.begin();
        
        let frame = self.window.get_size();
        
        if let Some(layout) = &mut self.layout {
            
            if frame.0 != 0 && frame.1 != 0 {
                layout.calculate(frame.0, frame.1, 10);
                
                let mut rng = rand::rng();
                for child in layout.iter()  {
                    self.renderer.draw_quad(
                        &Quad2D { 
                            x: child.computed.inner_pos.0, 
                            y: child.computed.inner_pos.1, 
                            width: child.computed.inner_dim.0, 
                            height: child.computed.inner_dim.1, 
                            color: Color { r: rng.random_range(0..256) as u8, g: rng.random_range(0..256) as u8, b: rng.random_range(0..256) as u8, a: 255 }
                        }
                    );
                };
            }
        }
        
        self.renderer.end()
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn show<T : LayoutElement + 'static>(&mut self, layout : T) {
        self.layout = Some(Box::new(layout));
        self.window.show();
    }

    fn poll(&mut self) -> Option<UserEvent> {
        self.window.poll_event()
    } 
}