use cgmath;


pub struct Camera {
  eye: cgmath::Point3<f32>,
  target: cgmath::Point3<f32>,
  up: cgmath::Vector3<f32>,

  aspect: f32,
  fovy: f32,
  znear: f32,
  zfar: f32,
}


#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
  1.0, 0.0, 0.0, 0.0,
  0.0, 1.0, 0.0, 0.0,
  0.0, 0.0, 0.5, 0.5,
  0.0, 0.0, 0.0, 1.0,
);
 
impl Camera {

  pub fn new(width : u32, height : u32) -> Self {
      Camera { 
        eye: (0.0, 1.0, 2.0).into(), 
        target: (0.0, 0.0, 0.0).into(), 
        up: cgmath::Vector3::unit_y(), 
        aspect: width as f32 / height as f32, 
        fovy: 45.0, 
        znear: 0.1, 
        zfar: 100.0
      }
  }

  pub fn resize(&mut self, width : u32, height : u32) {
    self.aspect = width as f32 / height as f32
  }

  pub fn projection(&self) -> cgmath::Matrix4<f32> {
    let view = cgmath::Matrix4::look_at_rh(self.eye, self.target,self.up);

    let proj = cgmath::perspective(cgmath::Deg(self.fovy),self.aspect, self.znear, self.zfar);

    return OPENGL_TO_WGPU_MATRIX * proj * view;
  }

}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
  pub fn new() -> Self {
    use cgmath::SquareMatrix;
    Self {
        view_proj: cgmath::Matrix4::identity().into(),
    }
  }

  pub fn update_view_proj(&mut self, camera: &Camera) {
    self.view_proj = camera.projection().into();
  }
}
 