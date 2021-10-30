use js_sys::WebAssembly::Memory;
use serde::__private::doc;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, Request, RequestInit, RequestMode, Response};
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation};

mod Mesh;
mod ObjLoader;

mod core;

use crate::core::EngineCore::EngineCore;
use crate::core::Shader::Shader;

mod gl {
    use wasm_bindgen::JsCast;
    use web_sys::WebGl2RenderingContext;
    use web_sys::{console, Request, RequestInit, RequestMode, Response};
    use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation};

    fn init() -> WebGl2RenderingContext {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let canvas = document.query_selector("canvas").unwrap().unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .map_err(|_| ())
            .unwrap();

        let width = window.inner_width().unwrap().as_f64().unwrap() as usize;
        let height = window.inner_height().unwrap().as_f64().unwrap() as usize;

        canvas.set_width(width as u32);
        canvas.set_height(height as u32);

        gl
    }

    thread_local! {
        pub static GL: WebGl2RenderingContext = init();
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub name: String,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    // console::log_1(foo()?);
    console::log_1(&"Hello using web-sys".into());

    Ok(())
}

pub fn initShader() -> Shader {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let vertex_shader = document
        .get_element_by_id("vertexShader")
        .unwrap()
        .inner_html();
    let fragment_shader = document
        .get_element_by_id("fragmentShader")
        .unwrap()
        .inner_html();

    console::log_1(&fragment_shader.clone().into());

    Shader::new(vertex_shader, fragment_shader)
}

#[wasm_bindgen]
pub async fn foo() -> Result<(), JsValue> {
    let shader = initShader();

    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");

    const URL: &str = "LP1.obj";
    // const URL: &str = "https://607ea67e02a23c0017e8bcd6.mockapi.io/foo";
    let request = web_sys::Request::new_with_str_and_init(&URL, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // let json = JsFuture::from(resp.json()?).await?;
    // let branch_into: Vec<Data> = json.into_serde().unwrap();

    // for i in branch_into {
    //     console::log_1(&(i.name.to_string()).into());
    // }

    // Ok(JsValue::from_serde(&branch_into).unwrap())

    let text: String = JsFuture::from(resp.text()?).await?.into_serde().unwrap();

    let mut loader = ObjLoader::Loader::new();

    loader.parse(&text);

    let mut meshes = loader.getMeshes();

    shader.enable();

    for mesh in meshes {
        mesh.render(&shader);
    }

    // unsafe {
    // let size = loader.vertices[10].x;
    // console::log_1(&size.into());
    // console::log_1(&meshes.len().to_string().into());
    // }

    Ok(())
}

// pub async fn bar() -> Result<JsValue, JsValue> {
//     let promise = js_sys::Promise::resolve(&42.into());
//     let result  = wasm_bindgen_futures::JsFuture::from(promise).await?;
//     Ok(result)
// }
