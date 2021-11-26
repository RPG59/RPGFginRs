use cgmath::Point3;
use serde::{Deserialize, Serialize};
use std::lazy::Lazy;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::WebGl2RenderingContext;
use web_sys::{console, Request, RequestInit, RequestMode, Response};
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation};
use ObjLoader::LoaderMesh;

mod Mesh;
mod ObjLoader;

mod core;

use crate::core::EngineCore::EngineCore;
use crate::core::Shader::Shader;

#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::sync::Mutex;

use std::collections::HashMap;

static mut WIDTH: usize = 0;
static mut HEIGHT: usize = 0;

lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();

        m.insert(0, "foo");

        m
    };
    static ref COUNT: usize = HASHMAP.len();
}

struct WebGLContext {
    context: RefCell<WebGl2RenderingContext>,
}

unsafe impl Sync for WebGLContext {}
unsafe impl Send for WebGLContext {}
unsafe impl Sync for WebGl2RenderingContext {}

pub fn init_context() -> WebGl2RenderingContext {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.query_selector("canvas").unwrap().unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(CANVAS_SIZE.width);
    canvas.set_height(CANVAS_SIZE.height);

    let gl = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .map_err(|_| ())
        .unwrap();

    gl
}

struct Canvas_size {
    width: u32,
    height: u32,
}

fn get_canvas_size() -> Canvas_size {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window.inner_height().unwrap().as_f64().unwrap() as u32;

    Canvas_size { width, height }
}

lazy_static! {
    // static ref WIDTH_woo: usize = {
    //     let window = web_sys::window().unwrap();
    //     let document = window.document().unwrap();
    //     window.inner_width().unwrap().as_f64().unwrap() as usize;
    // };
    static ref CANVAS_SIZE: Canvas_size = get_canvas_size();

    // static ref GL: WebGLContext = WebGLContext{
    //     context: RefCell::new(init_context())
    // };
    static ref GL: Lazy<WebGl2RenderingContext> = Lazy::new(||{
        init_context()
    });

        // let window = web_sys::window().unwrap();
        // let document = window.document().unwrap();

        // unsafe {
        //     WIDTH = window.inner_width().unwrap().as_f64().unwrap() as usize;
        //     HEIGHT = window.inner_height().unwrap().as_f64().unwrap() as usize;

        //     let canvas = document.query_selector("canvas").unwrap().unwrap();
        //     let canvas: HtmlCanvasElement = canvas
        //         .dyn_into::<HtmlCanvasElement>()
        //         .map_err(|_| ())
        //         .unwrap();

        //     canvas.set_width(WIDTH as u32);
        //     canvas.set_height(HEIGHT as u32);

        //     let gl = canvas
        //         .get_context("webgl2")
        //         .unwrap()
        //         .unwrap()
        //         .dyn_into::<WebGl2RenderingContext>()
        //         .map_err(|_| ())
        //         .unwrap();

        // }
}

// mod gl {
//     use wasm_bindgen::JsCast;
//     use web_sys::WebGl2RenderingContext;
//     use web_sys::{console, Request, RequestInit, RequestMode, Response};
//     use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation};
//     use crate::{HEIGHT, WIDTH};

//     fn init() -> WebGl2RenderingContext {
//         let window = web_sys::window().unwrap();
//         let document = window.document().unwrap();

//         unsafe {
//             WIDTH = window.inner_width().unwrap().as_f64().unwrap() as usize;
//             HEIGHT = window.inner_height().unwrap().as_f64().unwrap() as usize;

//         let canvas = document.query_selector("canvas").unwrap().unwrap();
//         let canvas: HtmlCanvasElement = canvas
//             .dyn_into::<HtmlCanvasElement>()
//             .map_err(|_| ())
//             .unwrap();

//         canvas.set_width(WIDTH as u32);
//         canvas.set_height(HEIGHT as u32);

//         let gl = canvas
//             .get_context("webgl2")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<WebGl2RenderingContext>()
//             .map_err(|_| ())
//             .unwrap();

//         gl
//         }
//     }

//     thread_local! {
//         pub static GL: WebGl2RenderingContext = init();
//     }
// }

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

struct foo_bar {
    size: u32,
    data: Vec<f32>,
}

#[wasm_bindgen]
pub fn getVertices() -> *mut u8 {
    let data = Box::new(foo_bar {
        size: 3,
        data: vec![2.234, 3.66, 4.6],
    });

    let ptr = Box::into_raw(data);

    ptr as *mut u8
}

#[wasm_bindgen]
pub async fn foo() -> Result<JsValue, JsValue> {
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
    // let vertices: Vec<f32> = vec!(
    //     -1., -1., 0.,
    //     1., 1., 0.,
    //      1., -1., 0.,
    //      -1., 1., 0.,);

    // let indices: Vec<u32> = vec!(0, 1, 2, 3, 0, 1 );

    // let _m = Mesh::Mesh::new(vertices, indices, Vec::new(), Vec::new());

    let mut camera = core::Camera::Camera::new(
        std::f32::consts::PI / 2.0,
        (CANVAS_SIZE.width / CANVAS_SIZE.height) as f32,
        0.1,
        10.,
        Point3::new(0., 0., -1.),
    );

    camera.update_proj_matrix();
    // let porj_matrix = camera.get_proj_matrix();

    shader.set_uniform_matrix4f(&"u_projMatrix".to_string(), &camera.get_proj_matrix());
    shader.set_uniform_matrix4f(&"u_viewMatrix".to_string(), &camera.get_view_matrix());

    let gl = GL.context.borrow();
    console::log_1(&"12341234".to_string().into());
    gl.clear_color(0., 1., 0., 1.);
    gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    // _m.render(&shader);

    for mesh in meshes {
        mesh.render(&shader);
    }

    // gl::GL.with(|gl| {
    //     gl.clear_color(255., 0., 0., 0.);
    // })

    // unsafe {
    // let size = loader.vertices[10].x;
    // console::log_1(&size.into());
    // console::log_1(&meshes.len().to_string().into());
    // }

    let data = HASHMAP.get(&0).unwrap();

    Ok((JsValue::from_str(data)))
}

// pub async fn bar() -> Result<JsValue, JsValue> {
//     let promise = js_sys::Promise::resolve(&42.into());
//     let result  = wasm_bindgen_futures::JsFuture::from(promise).await?;
//     Ok(result)
// }
