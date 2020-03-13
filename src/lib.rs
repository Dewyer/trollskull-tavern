mod utils;
mod hengine;
mod document;
extern crate web_sys;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("fasz");

	web_sys::console::log_1(&"geci".into());

	body.append_child(&val)?;

    Ok(())
}
