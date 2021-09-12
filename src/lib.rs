use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response, console};


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

#[wasm_bindgen]
pub async fn foo() -> Result<(), JsValue> {
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
    for (idx, line) in text.lines().enumerate() {
        // let (line, mut words) = match line {
        //     Ok(ref line) => (line.clone(), line.split_whitespace().filter(|s| !s.is_empty())),
        //     Err(err) => {

        //     }
        // };

        let first = line.split_whitespace().filter(|x| !x.is_empty());

        for fff in first {
            console::log_1(&(fff).into());
        }


        // console::log_1(&(first).into());
        // console::log_1(&line.into());
        // console::log_1(&(first.unwrap().to_string()).into());
    }


    Ok(())
}

// pub async fn bar() -> Result<JsValue, JsValue> {
//     let promise = js_sys::Promise::resolve(&42.into());
//     let result  = wasm_bindgen_futures::JsFuture::from(promise).await?;
//     Ok(result)
// }