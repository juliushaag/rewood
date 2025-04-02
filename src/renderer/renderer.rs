use std::sync::Arc;

use winit::window::Window;

use wgpu::{DeviceDescriptor, Instance, RequestAdapterOptions, TextureViewDescriptor};


pub trait RenderStage {
    fn render(&mut self, encoder : & mut wgpu::CommandEncoder, view : &wgpu::TextureView);
}
 
pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    window: Arc<Window>,
    config : wgpu::SurfaceConfiguration,
    stages : Vec<Box<dyn RenderStage>>
}

impl Renderer {
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Arc<Window>) -> Renderer {
        let instance = Instance::default();


        let surface = instance.create_surface(window.clone()).expect("Failed to create a window surface");

        let adapter_opt = RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..RequestAdapterOptions::default()
        };

        let adapter = instance.request_adapter(&adapter_opt).await.expect("Failed to create rendering adapter");


        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };



        let device_descr = DeviceDescriptor::default();

        let (device, queue) = adapter.request_device(&device_descr, None).await.expect("Failed to create rendering device");

        window.request_redraw();

        Renderer {
            surface,
            device,
            queue,
            config,
            window: window.clone(),
            stages : Vec::new()
        }
    }

    pub fn add_stage(&mut self, stage : Box<dyn RenderStage>) {
        self.stages.push(stage);
    }

    pub fn resize(&mut self, width : u32, height : u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn create_encoder(&self) -> wgpu::CommandEncoder {
        self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("GUI Renderer Encoder")} )
    }


    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>{

        let mut encoder = self.create_encoder();
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());

        self.stages.iter_mut().for_each(| stage| { stage.render(&mut encoder, &view) });
      
        self.queue.submit(std::iter::once(encoder.finish()));

        output.present();
        
        Ok(())
    }
}
