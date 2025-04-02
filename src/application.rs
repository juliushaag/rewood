use std::sync::Arc;
use winit::{window::Window, event_loop::EventLoop};
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowBuilder;

use crate::renderer::renderer::Renderer;
use crate::renderer::geom_stage::GeomStage;

use crate::user_event::UserEvent;


pub trait Layer {
fn init(&mut self) -> Result<(), String>;


fn on_event(&mut self, event : UserEvent) -> Result<(), String>;


fn on_render(&mut self) -> Result<(), String>;
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
        let renderer = pollster::block_on(Renderer::new(&window));
        let (width, height) = (window.inner_size().width, window.inner_size().height);
        Application { window, renderer, size: (width, height), layers : Vec::new() }
    }

    pub fn add_layer(&mut self, layer : Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn init(&mut self) -> Result<(), String>{
        for layer in &mut self.layers {
            layer.init()?;
        }
        Ok(())
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
                    let _ = self.renderer.render();
                    self.window.request_redraw();
                    for layer in &mut self.layers {
                        layer.on_render().unwrap();
                    }
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