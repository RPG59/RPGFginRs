pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Vec2 {
    x: f32,
    y: f32,
    z: f32,
}

pub struct Object3D {
  name: String,
  indices: Vec<f32>,
  usemtl: String,
  tex_indeces: Vec<f32>,
  normal_indices: Vec<f32>,
}

pub struct Material {
  name: String,
  Ns: f32,
  Ni: f32,
  d: f32,
  Tr: f32,
  Tf: f32,
  Ka: Vec3,
  Kd: Vec3,
  Ks: Vec3,
  Ke: Vec3,
  map_Ka: String,
  map_Kd: String,
}


pub struct Obj_loader {
  objects: Vec<Object3D>,
  mtllib: Vec<String>,
  usemtl: Vec<String>,
  vertices: Vec<Vec3>,
  tex_coord: Vec<Vec2>,
  normals: Vec<Vec3>,
  indices: Vec<f32>,
  mtl_data: Vec<Material>,
}

impl Obj_loader {
  pub fn parse(text: String) {
    text.lines()
  }
}