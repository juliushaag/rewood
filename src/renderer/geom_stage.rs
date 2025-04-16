use wgpu::naga::proc::index;

use super::primitives::buffer::DynamicBuffer;
use super::renderer::RenderPass;

use super::renderer::RenderState;

use super::primitives::buffer::Vertex;
use super::renderer::SharedBindGroups;
// This stage clears the screen and renders all the necessary geometries as the foundation

pub struct GeomStage {
  pipeline : wgpu::RenderPipeline,
  vertex_buffer : DynamicBuffer<Vertex>,
  index_buffer : DynamicBuffer<u32>,
}

const GEOM_VERTICES: &[Vertex] = &[
  Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
  Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
  Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
  Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
  Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];
 
const GEOM_INDICES: &[u32] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];
impl RenderPass for GeomStage {

  fn new(renderer : &mut RenderState) -> Self {
   
    let render_pipeline_layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[
          &renderer.shared.global_layout
        ],
        push_constant_ranges: &[],
    });

    let shader = renderer.device.create_shader_module(wgpu::include_wgsl!("shaders/geom_shader.wgsl"));

    let render_pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Geom Stage Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
          module: &shader,
          entry_point: "vs_main", // 1.
          buffers: &[Vertex::description()], // 2.
          compilation_options: wgpu::PipelineCompilationOptions::default(),
      },
      fragment: Some(wgpu::FragmentState { // 3.
          module: &shader,
          entry_point: "fs_main",
          targets: &[Some(wgpu::ColorTargetState { // 4.
              format: renderer.config.format,
              blend: Some(wgpu::BlendState::REPLACE),
              write_mask: wgpu::ColorWrites::ALL,
          })],
          compilation_options: wgpu::PipelineCompilationOptions::default(),
      }),
        primitive: wgpu::PrimitiveState {
          topology: wgpu::PrimitiveTopology::TriangleList, // 1.
          strip_index_format: None,
          front_face: wgpu::FrontFace::Ccw, // 2.
          cull_mode: Some(wgpu::Face::Back),
          // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
          polygon_mode: wgpu::PolygonMode::Fill,
          // Requires Features::DEPTH_CLIP_CONTROL
          unclipped_depth: false,
          // Requires Features::CONSERVATIVE_RASTERIZATION
          conservative: false,
      },
      depth_stencil: None, // 1.
      multisample: wgpu::MultisampleState {
          count: 1, // 2.
          mask: !0, // 3.
          alpha_to_coverage_enabled: false, // 4.
      },
      multiview: None, // 5.
      cache: None, // 6.
    });


    let vertex_buffer = DynamicBuffer::new(&renderer.device, wgpu::BufferUsages::VERTEX, 1024 * 1024); // 1MB Vertex Buffer

    let index_buffer = DynamicBuffer::new(&renderer.device, wgpu::BufferUsages::INDEX, 1024 * 1024); // 1MB Vertex Buffer

    vertex_buffer.update(&renderer.queue, GEOM_VERTICES);
    index_buffer.update(&renderer.queue, GEOM_INDICES);

    GeomStage { pipeline: render_pipeline, vertex_buffer, index_buffer }
  }

  
  fn render(&mut self, renderer : &mut RenderState, encoder : & mut wgpu::CommandEncoder, view : &wgpu::TextureView) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("Geom Render Pass"),
      color_attachments: &[
          // This is what @location(0) in the fragment shader targets
          Some(wgpu::RenderPassColorAttachment {
              view: &view,
              resolve_target: None,
              ops: wgpu::Operations {
                  load: wgpu::LoadOp::Clear(
                      wgpu::Color {
                          r: 0.1,
                          g: 0.2,
                          b: 0.3,
                          a: 1.0,
                      }
                  ),
                  store: wgpu::StoreOp::Store,
              }
          })
      ],
      depth_stencil_attachment: None,
      ..wgpu::RenderPassDescriptor::default()  
    });

    let nind = GEOM_INDICES.len() as u32;

    render_pass.set_pipeline(&self.pipeline);
    render_pass.set_bind_group(0, &renderer.shared.global, &[]);

    render_pass.set_vertex_buffer(0, self.vertex_buffer.buffer().slice(..));
    render_pass.set_index_buffer(self.index_buffer.buffer().slice(..), wgpu::IndexFormat::Uint32); // 1.

    render_pass.draw_indexed(0..nind, 0, 0..1); // 2.
 
  }
  
}