use crate::gl::GL;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub struct IndexBuffer {
    buffer: WebGlBuffer,
}

impl IndexBuffer {
    pub fn new(data: Vec<u32>, usage: u32) -> Self {
        GL.with(|gl| {
            let buffer = gl.create_buffer().unwrap();

            gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));

            let memory_buffer = wasm_bindgen::memory()
                .dyn_into::<js_sys::WebAssembly::Memory>()
                .unwrap()
                .buffer();
            let data_location = data.as_ptr() as u32 / 4;
            let array = js_sys::Uint32Array::new(&memory_buffer)
                .subarray(data_location, data_location + data.len() as u32);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &array,
                usage,
            );

            IndexBuffer { buffer }
        })
    }

    pub fn bind(&self) {
        GL.with(|gl| {
            gl.bind_buffer(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                Some(&self.buffer),
            )
        })
    }

    pub fn unbind(&self) {
        // TODO!
    }
}
