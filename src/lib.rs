extern crate wasm_bindgen;
extern crate web_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("no `document` on window");
    let body = document.body().expect("no `body` on document");

    let p = document.create_element("p").expect("can not create `p` element on document");
    p.set_inner_html("Hello from rust !!");

    AsRef::<web_sys::Node>::as_ref(&body).append_child(p.as_ref()).expect("can not append `p` as a child of the body");

    log("Hello from rust !!");

    alert("Hello from rust !!");
}
