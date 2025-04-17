use std::{num::NonZeroU32, time::Instant};

use gl_rs::{types::GLint, GetIntegerv, FRAMEBUFFER_BINDING};
use rand::{rngs::StdRng, Rng, SeedableRng};
use skia_safe::{font_style::Width, gpu::{self, backend_render_targets, gl::{self, FramebufferInfo, UInt}, SurfaceOrigin}, Color, Color4f, ColorType, Paint, Surface};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::{Modifiers, WindowEvent}, event_loop::EventLoop, raw_window_handle::HasWindowHandle, window::WindowAttributes};

use crate::gui::layout::LayoutElement;

use glutin::{
  config::ConfigTemplateBuilder,
  context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
  display::GetGlDisplay,
  prelude::*,
  surface::{Rect, Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};

use winit::event_loop::{ActiveEventLoop, EventLoopBuilder };
use winit::window::Window;

pub struct Application {

  state : Option<ApplicationState>,
  
  layout : Option<Box<dyn LayoutElement>>
}


struct ApplicationState {
  surface: Surface,
  gl_surface: GlutinSurface<WindowSurface>,
  gr_context: skia_safe::gpu::DirectContext,
  gl_context: PossiblyCurrentContext,
  window: Window,

  fb_info: FramebufferInfo,
  num_samples: usize,
  stencil_size: usize,
  modifiers: Modifiers,
  frame: usize,
  previous_frame_start: Instant,
}

impl ApplicationHandler for Application {
  
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
      let attributes = Window::default_attributes()
        .with_title("Redwood")
        .with_inner_size(PhysicalSize::new(1280, 780))
        .with_visible(true);

      let template = ConfigTemplateBuilder::new();
      let (window, gl_config)  = glutin_winit::DisplayBuilder::new()
          .with_window_attributes(Some(attributes))
          .build(event_loop, template, |mut configs| configs.next().unwrap())
          .unwrap();

      let window = window.unwrap();
      let window_handle = window.window_handle().unwrap().as_raw();

      let gl_display = gl_config.display();
  
      let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(Some(window_handle));

      let not_current_gl_context = unsafe {
        gl_display.create_context(&gl_config, &context_attributes).unwrap()
      };

      let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
          window_handle,
          NonZeroU32::new(window.inner_size().width).unwrap(),
          NonZeroU32::new(window.inner_size().height).unwrap(),
      );
      
      
      let gl_surface = unsafe { gl_display.create_window_surface(&gl_config, &attrs).unwrap() };
      
      let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();
      

      gl_rs::load_with(|s| {
          let cstr = std::ffi::CString::new(s).unwrap();
          gl_display.get_proc_address(&cstr)
      });
  

      let interface = gpu::gl::Interface::new_load_with(|s| {
          let cstr = std::ffi::CString::new(s).unwrap();
          gl_display.get_proc_address(&cstr) as _
      }).unwrap();

      let mut gr_context  = gpu::direct_contexts::make_gl(interface, None).unwrap();
       // Framebuffer info
      
      let fb_info = {
        let mut fboid: GLint = 0;
        unsafe { GetIntegerv(FRAMEBUFFER_BINDING, &mut fboid) };

        FramebufferInfo {
            fboid: fboid.try_into().unwrap(),
            format: skia_safe::gpu::gl::Format::RGBA8.into(),
            ..Default::default()
        }
      };
      // Get surface dimensions
      let size = window.inner_size();
      let backend_render_target = gpu::backend_render_targets::make_gl(
          (size.width as i32, size.height as i32),
          0, // sample count
          0, // stencil bits
          fb_info,
      );

      // Create Skia surface
      let mut surface = gpu::surfaces::wrap_backend_render_target(
          &mut gr_context,
          &backend_render_target,
          gpu::SurfaceOrigin::BottomLeft,
          ColorType::RGBA8888,
          None,
          None,
      )
      .unwrap();

      let canvas = surface.canvas();
      
      window.request_redraw();

      let state = ApplicationState {
        window: window,
        surface: surface,
        gl_context: gl_context,
        gl_surface: gl_surface,
        gr_context: gr_context,

        fb_info: fb_info,
        num_samples: gl_config.num_samples() as usize,
        stencil_size: gl_config.stencil_size() as usize,
        modifiers: Modifiers::default(),
        frame: 0,
        previous_frame_start: Instant::now(),
      };

      self.state = Some(state);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
      match event {
        WindowEvent::CloseRequested => event_loop.exit(),
        WindowEvent::Resized(size) => {

          if self.state.is_none() { return }
          
          let (width, height): (u32, u32) = size.into();

          let new_skia_surface = self.create_surface(width, height);
          let state = self.state.as_mut().unwrap();
          state.surface = new_skia_surface;

          /* First resize the opengl drawable */

          state.gl_surface.resize(
              &state.gl_context,
              NonZeroU32::new(width.max(1)).unwrap(),
              NonZeroU32::new(height.max(1)).unwrap(),
          );

          self.state.as_mut().unwrap().window.request_redraw();
        }

        WindowEvent::RedrawRequested => {
          self.update();
          self.state.as_mut().unwrap().window.request_redraw();
        },
        _ => ()
      }
    }
}


impl Application {

  

  
  pub fn new<T : LayoutElement + 'static>(layout : T) -> Self { Application { state: None, layout: Some(Box::new(layout))} }

  pub fn run(mut self) {

    if let Some(layout) = &mut self.layout {
      for child in layout.iter_mut()  {
        child.update_unit_size(10);
      }
    }

    let event_loop = EventLoopBuilder::default().build().unwrap();
    event_loop.run_app(&mut self).unwrap();
  }

  fn create_surface(
    &mut self,
    width : u32,
    height: u32,
  ) -> Surface {
    
      let num_samples = self.state.as_ref().unwrap().num_samples;
      let stencil_bits = self.state.as_ref().unwrap().stencil_size;
      let fb_info = self.state.as_ref().unwrap().fb_info;

      let backend_render_target = 
          backend_render_targets::make_gl((width as i32, height as i32), num_samples, stencil_bits, fb_info);

      gpu::surfaces::wrap_backend_render_target(
          &mut self.state.as_mut().unwrap().gr_context,
          &backend_render_target,
          SurfaceOrigin::BottomLeft,
          ColorType::RGBA8888,
          None,
          None,
      )
      .expect("Could not create skia surface")
  }

  pub fn update(&mut self) {
    
    let state = self.state.as_mut().unwrap();


    state.frame += 1;
    let canvas = state.surface.canvas();
    canvas.clear(Color::WHITE);
    let frame : (u32, u32)= state.window.inner_size().into();

    let mut rng = StdRng::seed_from_u64(10);

    if let Some(layout) = &mut self.layout {
        
        if frame.0 != 0 && frame.1 != 0 {
            layout.calculate(frame.0, frame.1, 10);

            for child in layout.iter()  {
                let paint  = Paint::new(Color4f::new(rng.random_range(0..255) as f32 / 255.0, rng.random_range(0..255) as f32 / 255.0, rng.random_range(0..255) as f32 / 255.0, 1.0), None);
                let layout = child.layout();
                canvas.draw_rect(
                  skia_safe::Rect::new(
                    layout.computed.inner_pos.0 as f32,
                    layout.computed.inner_pos.1 as f32,
                    (layout.computed.inner_pos.0 + layout.computed.inner_dim.0) as f32,
                    (layout.computed.inner_pos.1 + layout.computed.inner_dim.1) as f32,
                  ),
                  &paint
                );
            };
        }
    }


    state.gr_context.flush_and_submit();
    state.gl_surface
          .swap_buffers(&state.gl_context)
          .unwrap();   
      
  }
 
}