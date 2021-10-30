use crate::gl::GL;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub struct StaticBuffer {
    buffer: WebGlBuffer,
}

impl StaticBuffer {
    pub fn new(data: Vec<f32>, usage: u32) -> Self {
        GL.with(|gl| {
            let buffer = StaticBuffer {
                buffer: gl.create_buffer().unwrap(),
            };
            buffer.set_data(data, usage);

            buffer
        })
    }

    pub fn bind(&self) {
        GL.with(|gl| gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer)))
    }

    pub fn unbind(&self) {
        // TODO!
    }

    fn set_data(&self, data: Vec<f32>, usage: u32) {
        use wasm_bindgen::JsCast;
        self.bind();
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<js_sys::WebAssembly::Memory>()
            .unwrap()
            .buffer();
        let data_location = data.as_ptr() as u32 / 4;
        let array = js_sys::Float32Array::new(&memory_buffer)
            .subarray(data_location, data_location + data.len() as u32);

        GL.with(|gl| {
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &array,
                usage,
            )
        });

        self.unbind();
    }
}
