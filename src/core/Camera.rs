use cgmath::{Matrix4, PerspectiveFov, Rad, SquareMatrix, Vector3, Point3};

pub struct Camera {
  pub proj_matrix: Matrix4<f32>,
  yaw: f32,
  pinch: f32,
  right: Vector3<f32>,
  front: Vector3<f32>,
  position: Point3<f32>,
  fov_rad: f32,
  aspect: f32,
  near: f32,
  far: f32,
} 

impl Camera {
  pub fn new(fov_rad: f32, aspect: f32, near: f32, far: f32, position: Point3<f32>) -> Self {
    Camera {
      proj_matrix: Matrix4::identity(),
      yaw: 0.,
      pinch: 0.,
      right: Vector3::new(0., 0., 0.),
      front: Vector3::new(0., 0., -1.),
      position,
      fov_rad,
      aspect,
      near,
      far
    }
  }

  pub fn update_proj_matrix(&mut self) {
      self.proj_matrix = Matrix4::from(PerspectiveFov{fovy: Rad(self.fov_rad), aspect: self.aspect, far: self.far, near: self.near});
  }

  pub fn get_view_matrix(&self) -> Matrix4<f32> {
    Matrix4::look_at_rh(self.position, Point3::new(0., 0., 0.), Vector3::new(0., 1., 0.))
  }

  pub fn get_proj_matrix(&self) -> Matrix4<f32> {
    self.proj_matrix.clone()
  }

}