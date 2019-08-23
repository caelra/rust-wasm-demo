extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;
    
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("WebAssembly in {}!", name));
}

#[wasm_bindgen]
pub fn dimmadome() {
    let div = document.createElement("div");
    let p   = document.createElement("p");

    p.set_inner("This is WebAssembly ¯|_(ツ)_/¯");
    div.append(p);

    document.body().append(div);
}