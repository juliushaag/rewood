use std::sync::Arc;
use winit::{window::Window, event_loop::EventLoop};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowBuilder;

use crate::renderer::renderer::Renderer;
use crate::renderer::geom_stage::GeomStage;

use crate::user_event::UserEvent;


pub trait Layer {
    fn new(app : &mut Application) -> Self where Self : Sized;


    fn on_event(&mut self, event : UserEvent) -> Result<(), String>;


    fn on_update(&mut self) -> Result<(), String>;
}

pub struct Application {
    window: Arc<Window>,
    renderer: Renderer,
    size: (u32, u32),

    layers : Vec<Box<dyn Layer>>
}


impl Application {

    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
        let mut renderer = Renderer::new(&window);

        renderer.stage::<GeomStage>();

        let (width, height) = (window.inner_size().width, window.inner_size().height);
        Application { window, renderer, size: (width, height), layers : Vec::new() }
    }

    pub fn attach<T : Layer + 'static>(&mut self) {
        let layer = Box::new(T::new(self));
        self.layers.push(layer);
    }

    pub fn render(&mut self) {
        self.renderer.render();
    }

    pub fn event_handler(&mut self, event : Event<()>, control_flow: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => match event {
                WindowEvent::RedrawRequested => {
                    self.render();
                    self.window.request_redraw();
                },
                WindowEvent::Resized(new_size) => {
                    if new_size.width > 0 && new_size.height > 0 {
                        self.size = (new_size.width, new_size.height);
                        self.renderer.resize(self.size.0, self.size.1);
                        self.window.request_redraw();
                    }

                }
                WindowEvent::CloseRequested => control_flow.exit(),
                _ => {}
            },
            _ => {}
        }
    }
}