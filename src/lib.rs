use cgmath;
use cgmath::InnerSpace;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::Document;
use web_sys::WebGl2RenderingContext;
use web_sys::WebGlBuffer;
use web_sys::{console, Request, RequestInit, RequestMode, Response};
use web_sys::{HtmlCanvasElement, WebGlProgram, WebGlShader, WebGlUniformLocation};

mod Mesh;
mod ObjLoader;
use crate::core::Types::{float3, float4x4, to_slice};

mod core;

#[macro_use]
extern crate lazy_static;

use std::cell::RefCell;
use std::collections::btree_set::Intersection;
use std::iter::Sum;

static mut WIDTH: usize = 0;
static mut HEIGHT: usize = 0;
struct WebGLContext {
    context: RefCell<WebGl2RenderingContext>,
}

unsafe impl Sync for WebGLContext {}
unsafe impl Send for WebGLContext {}

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
pub fn foo(text: &js_sys::Uint8Array) -> js_sys::Object {
    // let mut opts = web_sys::RequestInit::new();
    // opts.method("GET");

    // const URL: &str = "LP1.obj";
    // // const URL: &str = "https://607ea67e02a23c0017e8bcd6.mockapi.io/foo";
    // let request = web_sys::Request::new_with_str_and_init(&URL, &opts)?;

    // let window = web_sys::window().unwrap();
    // let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // assert!(resp_value.is_instance_of::<Response>());
    // let resp: Response = resp_value.dyn_into().unwrap();

    // // let json = JsFuture::from(resp.json()?).await?;
    // // let branch_into: Vec<Data> = json.into_serde().unwrap();

    // // for i in branch_into {
    // //     console::log_1(&(i.name.to_string()).into());
    // // }

    // // Ok(JsValue::from_serde(&branch_into).unwrap())

    // let text: String = JsFuture::from(resp.text()?).await?.into_serde().unwrap();

    let mut loader = ObjLoader::Loader::new();
    let bytes_vec = text.to_vec();
    let str_data = std::str::from_utf8(&bytes_vec).unwrap().to_string();

    loader.parse(&str_data);

    let res = js_sys::Object::new();

    js_sys::Reflect::set(&res, &"meshes".into(), &loader.getMeshes());
    js_sys::Reflect::set(&res, &"mtlData".into(), &loader.getMtlLib());

    res
}

fn float3_to_js(data: &float3) -> js_sys::Object {
    let res = js_sys::Object::new();

    js_sys::Reflect::set(&res, &"x".into(), &data.x.into());
    js_sys::Reflect::set(&res, &"y".into(), &data.y.into());
    js_sys::Reflect::set(&res, &"z".into(), &data.z.into());

    res
}

fn normalize(v: float3) -> float3 {
    let mut len = f32::sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
    len = if len.abs() < f32::EPSILON { 1.0 } else { len };

    float3::new(v.x / len, v.y / len, v.z / len)
}

fn check_intersection(
    v0: float3,
    v1: float3,
    v2: float3,
    origin: float3,
    direction: float3,
) -> Option<js_sys::Object> {
    let edge0 = v1 - v0;
    let edge1 = v2 - v0;
    let N = normalize(edge0.cross(edge1));
    let NdotDir = direction.dot(N);

    if NdotDir.abs() < 0.0001 {
        return None;
    }

    let t = (v0 - origin).dot(N) / NdotDir;

    if t < 0. {
        return None;
    }

    let intersection_point = origin + direction * t;

    let vp0 = intersection_point - v0;

    if edge0.cross(vp0).dot(N) < 0. {
        return None;
    }

    let vp1 = intersection_point - v1;

    if (v2 - v1).cross(vp1).dot(N) < 0. {
        return None;
    }

    let vp2 = intersection_point - v2;

    if (v0 - v2).cross(vp2).dot(N) < 0. {
        return None;
    }

    let res = js_sys::Object::new();

    js_sys::Reflect::set(&res, &"distance".into(), &t.into());
    js_sys::Reflect::set(&res, &"normal".into(), &float3_to_js(&N));
    js_sys::Reflect::set(
        &res,
        &"intersectionPoint".into(),
        &float3_to_js(&intersection_point),
    );

    Some(res)
}

#[wasm_bindgen]
pub fn raycast(
    js_data: js_sys::Float32Array,
    js_origin: js_sys::Float32Array,
    js_direction: js_sys::Float32Array,
) -> js_sys::Array {
    let data = js_data.to_vec();
    let origin_vec = js_origin.to_vec();
    let direction_vec = js_direction.to_vec();
    let origin = float3::new(origin_vec[0], origin_vec[1], origin_vec[2]);
    let direction = float3::new(direction_vec[0], direction_vec[1], direction_vec[2]);
    let intersections = js_sys::Array::new();

    let mut i = 0;

    while i < data.len() {
        let v0 = float3::new(data[i + 0], data[i + 1], data[i + 2]);
        let v1 = float3::new(data[i + 3], data[i + 4], data[i + 5]);
        let v2 = float3::new(data[i + 6], data[i + 7], data[i + 8]);
        let intersection_data = check_intersection(v0, v1, v2, origin, direction);

        // intersections.push(&intersection_data.unwrap());

        if intersection_data.is_some() {
            intersections.push(&intersection_data.unwrap());
        }

        i += 9;
    }

    intersections
}

// pub async fn bar() -> Result<JsValue, JsValue> {
//     let promise = js_sys::Promise::resolve(&42.into());
//     let result  = wasm_bindgen_futures::JsFuture::from(promise).await?;
//     Ok(result)
// }

// #[wasm_bindgen]
// pub fn create_vertex_buffer(gl: &WebGl2RenderingContext, data: Vec<f32>) -> WebGlBuffer {
//     let buffer_id = gl.create_buffer();
//     let memory_buffer = wasm_bindgen::memory()
//         .dyn_into::<js_sys::WebAssembly::Memory>()
//         .unwrap()
//         .buffer();
//     let data_location = data.as_ptr() as u32 / 4;
//     let array = js_sys::Float32Array::new(&memory_buffer)
//         .subarray(data_location, data_location + data.len() as u32);

//     gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer_id);
//     gl.buffer_data_with_array_buffer_view(
//         WebGl2RenderingContext::ARRAY_BUFFER,
//         &array,
//         WebGl2RenderingContext::STATIC_DRAW,
//     );

//     buffer_id.unwrap()
// }
