use bytemuck;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Zeroable, bytemuck::Pod)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 3],
}

impl Vertex {
  const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
        
  pub fn description() ->  wgpu::VertexBufferLayout<'static> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::VertexStepMode::Vertex,
      attributes: &Self::ATTRIBS,
    } 
  }
}

use wgpu::{Buffer, BufferUsages, Device, Queue};
use bytemuck::{Pod, Zeroable}; // For safe byte conversions

pub struct DynamicBuffer<T: Pod + Zeroable> {
    buffer: Buffer,
    size: usize, // In number of elements
    usage: BufferUsages,
    _marker: std::marker::PhantomData<T>, // Marker to use T
}

impl<T: Pod + Zeroable> DynamicBuffer<T> {
    pub fn new(device: &Device, usage: BufferUsages, initial_size: usize) -> Self {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Dynamic Buffer"),
            size: (std::mem::size_of::<T>() * initial_size) as u64,
            usage: usage | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            buffer,
            size: initial_size,
            usage,
            _marker: std::marker::PhantomData::<T> {}
        }
    }

    /// Resize the buffer if needed (buffer doubling strategy)
    pub fn ensure_capacity(&mut self, device: &Device, new_size: usize) {
        if new_size > self.size {
            let new_alloc_size = new_size.max(self.size * 2); // Double size
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Resized Dynamic Buffer"),
                size: (std::mem::size_of::<T>() * new_alloc_size) as u64,
                usage: self.usage | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.size = new_alloc_size;
        }
    }

    /// Upload new data to GPU
    pub fn update(&self, queue: &Queue, data: &[T]) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(data));
    }

    /// Get the underlying wgpu buffer
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

}
