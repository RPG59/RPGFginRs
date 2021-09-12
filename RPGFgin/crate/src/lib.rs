#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    // ...
    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));
    // ...

    Ok(())
}