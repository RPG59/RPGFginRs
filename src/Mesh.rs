// use crate::GL;
// use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

// use crate::core::{IndexBuffer::IndexBuffer, Shader::Shader, StaticBuffer::StaticBuffer};

// pub struct Mesh {
//     VBO: StaticBuffer,
//     IBO: IndexBuffer,
//     VAO: WebGlVertexArrayObject,
//     numberOfIndices: usize,
// }

// impl Mesh {
//     pub fn new(
//         vertices: Vec<f32>,
//         indices: Vec<u32>,
//         texCoords: Vec<f32>,
//         normals: Vec<f32>,
//     ) -> Mesh {
//         let gl = GL.context.borrow();
//         let VAO = gl.create_vertex_array().unwrap();
//         gl.bind_vertex_array(Some(&VAO));

//         let VBO = StaticBuffer::new(vertices, WebGl2RenderingContext::STATIC_DRAW);

//         gl.enable_vertex_attrib_array(0);
//         gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);

//         // gl.enable_vertex_attrib_array(1);
//         // gl.vertex_attrib_pointer_with_i32(1, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);

//         // gl.enable_vertex_attrib_array(2);
//         // gl.vertex_attrib_pointer_with_i32(2, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);

//         let numberOfIndices = indices.len();
//         let IBO = IndexBuffer::new(indices, WebGl2RenderingContext::STATIC_DRAW);

//         Mesh {
//             numberOfIndices,
//             VBO,
//             IBO,
//             VAO,
//         }
//     }
//     pub fn render(self, shader: &Shader) {
//         let gl = GL.context.borrow();
//         gl.bind_vertex_array(Some(&self.VAO));

//         gl.draw_elements_with_i32(
//             WebGl2RenderingContext::TRIANGLES,
//             self.numberOfIndices as i32,
//             WebGl2RenderingContext::UNSIGNED_INT,
//             0,
//         )
//     }
// }
