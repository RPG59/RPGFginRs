use core::panic;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::{net::Incoming, str::SplitWhitespace};
use web_sys::console;

use crate::Mesh::Mesh;

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Self { x, y, z }
    }
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Self { x, y }
    }
}

impl<T> std::ops::Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            n => panic!("Invalid index Vec3, {}", n),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            n => panic!("Invalid index Vec3, {}", n),
        }
    }
}

impl<T> std::ops::Index<usize> for Vec2<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        match index {
            0 => &self.x,
            1 => &self.y,
            n => panic!("Invalid index Vec2, {}", n),
        }
    }
}

impl<T> std::ops::IndexMut<usize> for Vec2<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            n => panic!("Invalid index Vec2, {}", n),
        }
    }
}

pub struct Object3D {
    pub name: String,
    pub indices: Vec<u32>,
    pub usemtl: String,
    pub tex_indeces: Vec<u32>,
    pub normal_indices: Vec<u32>,
}

impl Object3D {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            indices: Vec::new(),
            usemtl: String::new(),
            tex_indeces: Vec::new(),
            normal_indices: Vec::new(),
        }
    }
}

pub struct Material {
    name: String,
    Ns: f32,
    Ni: f32,
    d: f32,
    Tr: f32,
    Tf: f32,
    Ka: Vec3<f32>,
    Kd: Vec3<f32>,
    Ks: Vec3<f32>,
    Ke: Vec3<f32>,
    map_Ka: String,
    map_Kd: String,
}

struct VertexIndex {
    pub v: usize,
    pub uv: usize,
    pub n: usize,
}

// impl VertexIndex {
// parse(face: &str, )
// }

pub struct Loader {
    objects: Vec<RefCell<Object3D>>,
    mtllib: Vec<String>,
    usemtl: Vec<String>,
    pub vertices: Vec<Vec3<f32>>,
    tex_coord: Vec<Vec2<f32>>,
    normals: Vec<Vec3<f32>>,
    indices: Vec<f32>,
    mtl_data: Vec<Material>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            mtllib: Vec::new(),
            usemtl: Vec::new(),
            vertices: Vec::new(),
            tex_coord: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
            mtl_data: Vec::new(),
        }
    }

    // TODO: interface form VecX
    fn parse_vec3(words: &mut SplitWhitespace, res: &mut Vec3<f32>) -> bool {
        for i in 0..3 {
            match words.next().unwrap().parse::<f32>() {
                Ok(x) => res[i] = x,
                Err(_) => return false,
            }
        }

        true
    }

    fn parse_vec2(words: &mut SplitWhitespace, res: &mut Vec2<f32>) -> bool {
        for i in 0..2 {
            match words.next().unwrap().parse::<f32>() {
                Ok(x) => res[i] = x,
                Err(_) => return false,
            }
        }

        true
    }

    pub fn parse(mut self: &'_ mut Self, text: &String) {
        for (idx, line) in text.lines().enumerate() {
            // let (line, mut words) = match line {
            //     Ok(ref line) => (line.clone(), line.split_whitespace().filter(|s| !s.is_empty())),
            //     Err(err) => {

            //     }
            // };

            let mut line_words = line.split_whitespace();

            match line_words.next() {
                Some("#") => continue,

                Some("o") | Some("g") => {
                    self.objects
                        .push(RefCell::new(Object3D::new(line_words.next().unwrap())));
                }

                Some("v") => {
                    let mut v3 = Vec3::new(0.0, 0.0, 0.0);

                    if Loader::parse_vec3(&mut line_words, &mut v3) {
                        self.vertices.push(v3);
                    }
                }

                Some("vt") => {
                    let mut v2 = Vec2::new(0.0, 0.0);

                    if Loader::parse_vec2(&mut line_words, &mut v2) {
                        self.tex_coord.push(v2);
                    }
                }

                Some("vn") => {
                    let mut v3 = Vec3::new(0.0, 0.0, 0.0);

                    if Loader::parse_vec3(&mut line_words, &mut v3) {
                        self.normals.push(v3);
                    }
                }

                Some("mtllib") => {
                    self.mtllib.push(line_words.next().unwrap().to_string());
                }

                Some("usemtl") => {
                    self.objects.last().unwrap().borrow_mut().usemtl =
                        line_words.next().unwrap().to_string();
                }

                Some("f") => {
                    let mut indices: Vec<u32> = Vec::new();
                    let mut tex: Vec<u32> = Vec::new();
                    let mut normal: Vec<u32> = Vec::new();

                    for face_element in line_words {
                        for item in face_element.split("/").enumerate() {
                            if !item.1.is_empty() {
                                match item.1.parse::<u32>() {
                                    Ok(x) => match item.0 {
                                        0 => indices.push(x - 1),
                                        1 => tex.push(x - 1),
                                        2 => normal.push(x - 1),
                                        _ => panic!("Invalid face!"),
                                    },
                                    Err(_) => panic!("Invalid face!"),
                                }
                            }
                        }
                    }

                    self.objects
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .indices
                        .append(&mut indices);
                    self.objects
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .tex_indeces
                        .append(&mut tex);
                    self.objects
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .normal_indices
                        .append(&mut &mut normal);
                }

                Some(_) => {}
                _ => {}
            }

            // for fff in first {
            //     console::log_1(&(fff).into());
            // }

            // console::log_1(&(first).into());
            // console::log_1(&line.into());
            // console::log_1(&(first.unwrap().to_string()).into());
        }
    }

    pub fn getMeshes(self) -> Vec<Mesh> {
        const POSITION_SIZE: usize = 3;
        const TEX_POSITION_SIZE: usize = 2;

        unsafe {
            console::log_1(&"GET_MESHES".to_string().into());
        }

        let mut meshes: Vec<Mesh> = Vec::new();

        for object in self.objects {
            let indices_len = object.borrow().indices.len() as usize;
            let tex_indices_len = object.borrow().tex_indeces.len() as usize;

            let mut indices: Vec<u32> = Vec::with_capacity(indices_len);
            let mut vertices: Vec<f32> = Vec::with_capacity(indices_len * POSITION_SIZE);
            let mut vt: Vec<f32> = Vec::with_capacity(tex_indices_len * TEX_POSITION_SIZE);
            let mut vn: Vec<f32> = Vec::with_capacity(tex_indices_len * POSITION_SIZE);

            // TODO: Remove clone!
            for x in object.borrow().indices.clone().iter().enumerate() {
                let vertex = self.vertices.get(*x.1 as usize).unwrap();
                vertices.push(vertex.x);
                vertices.push(vertex.y);
                vertices.push(vertex.z);
                indices.push(x.0 as u32);
            }

            for x in object.borrow().tex_indeces.clone() {
                let tex = self.tex_coord.get(x as usize).unwrap();
                vt.push(tex.x);
                vt.push(tex.y);
            }

            for x in object.borrow().normal_indices.clone() {
                let normal = self.normals.get(x as usize).unwrap();
                vn.push(normal.x);
                vn.push(normal.y);
                vn.push(normal.z);
            }

            meshes.push(Mesh {
                indices,
                vertices,
                normals: vn,
                texCoords: vt,
            })
        }

        meshes
    }
}
