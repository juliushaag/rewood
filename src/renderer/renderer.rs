use std::sync::Arc;

use winit::window::Window;

use wgpu::{core::device, util::DeviceExt, Adapter, DeviceDescriptor, Instance, RequestAdapterOptions, TextureViewDescriptor};
use super::camera::{ Camera, CameraUniform };

use std::collections::HashMap;
 
pub struct SharedBindGroups {
  pub global : wgpu::BindGroup,
  pub global_layout : wgpu::BindGroupLayout
}

pub struct RenderState {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config : wgpu::SurfaceConfiguration,

    pub shared : SharedBindGroups
}

pub trait RenderPass {

    fn new(renderer : &mut RenderState) -> Self where Self : Sized;

    fn render(&mut self, renderer: &mut RenderState, encoder : & mut wgpu::CommandEncoder, view : &wgpu::TextureView);
}


pub struct Renderer {
  state: RenderState,
  camera: Camera,
  passes: Vec<Box<dyn RenderPass>>, 
  camera_buffer: wgpu::Buffer
}

impl Renderer {

  pub fn new(window: &Arc<Window>) -> Self {

    let instance = Instance::default();
    let surface  = instance.create_surface(window.clone()).expect("Failed to create a window surface");

    let adapter_opt = RequestAdapterOptions {
      compatible_surface: Some(&surface),
      ..RequestAdapterOptions::default()
    };

    let adapter = pollster::block_on(instance.request_adapter(&adapter_opt)).expect("Failed to create rendering adapter");

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

    let (device , queue) = pollster::block_on(adapter.request_device(&device_descr, None)).expect("Failed to create rendering device");


    window.request_redraw();


    let width = config.width.clone();
    let height = config.height.clone();

    let camera = Camera::new( width, height);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&camera);


    let camera_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        }
    );

    let global_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }
        ],

        label: Some("camera_bind_group_layout"),
    });

    let global_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &global_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }
        ],
        label: Some("global"),
    });

    let shared_bind_groups = SharedBindGroups { global: global_bind_group, global_layout: global_bind_group_layout };

    let state = RenderState { surface, device, queue, config, shared: shared_bind_groups };

    Self { state, camera, passes: Vec::new(), camera_buffer }
  } 

  pub fn stage<T: RenderPass + 'static>(&mut self) -> &mut Self {
    let stage = Box::new(T::new(&mut self.state));
    self.passes.push(stage);
    self
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    self.state.config.width = width;
    self.state.config.height = height;
    self.state.surface.configure(&self.state.device, &self.state.config);

    self.camera.resize(width, height);

    let mut camera_uniform = CameraUniform::new();
    camera_uniform.update_view_proj(&self.camera);

    self.state.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
  }

  pub fn render(&mut self) {

    let mut encoder = self.state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("GUI Renderer Encoder")} );
    

    let surface_texture = self.state.surface.get_current_texture().unwrap();
    let view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

    for pass in &mut self.passes {
      pass.render(&mut self.state, &mut encoder, &view);
    }

    self.state.queue.submit(std::iter::once(encoder.finish()));

    surface_texture.present();
  }

}