use crate::core::Types::{float4x4, to_slice};
use crate::GL;
use std::fs;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};

pub struct Shader {
    program: WebGlProgram,
    // uniforms: HashMap<String, WebGlUniformLocation>,
}

impl Shader {
    pub fn new(vs_data: String, fs_data: String) -> Self {
        let vs_shader = Shader::compile(vs_data, WebGl2RenderingContext::VERTEX_SHADER);
        let fs_shader = Shader::compile(fs_data, WebGl2RenderingContext::FRAGMENT_SHADER);

        Shader {
            program: Shader::creteProgram(&vs_shader, &fs_shader),
        }
    }

    fn get_shader_source(path: String) -> String {
        let data = fs::read_to_string(path).expect(("ERROR: [get_shader_source]: "));
        return data;
    }

    fn compile(source: String, shader_type: u32) -> WebGlShader {
            let gl = GL.context.borrow();
            let shader = gl.create_shader(shader_type).unwrap();

            gl.shader_source(&shader, &source);
            gl.compile_shader(&shader);

            if !gl
                .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
                .as_bool()
                .unwrap_or(false)
            {
                let log = gl.get_shader_info_log(&shader).unwrap();
                // console::log_1(&log.into());
            }

            shader
    }

    fn creteProgram(vs_shader: &WebGlShader, fs_shader: &WebGlShader) -> WebGlProgram {
            let gl = GL.context.borrow();
            let program = gl.create_program().unwrap();
            gl.attach_shader(&program, vs_shader);
            gl.attach_shader(&program, fs_shader);

            gl.delete_shader(Some(vs_shader));
            gl.delete_shader(Some(fs_shader));

            gl.link_program(&program);

            program
    }

    pub fn enable(&self) {
            let gl = GL.context.borrow();
            gl.use_program(Some(&self.program));
            gl.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0);
    }

    pub fn set_uniform_matrix4f(&self, name: &String, data: &float4x4) {
            let gl = GL.context.borrow();
            let location = gl.get_uniform_location(&self.program, name).unwrap();
            gl.uniform_matrix4fv_with_f32_array(Some(&location), false, &to_slice(data));
    }
}
