use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s : &str);
}

#[wasm_bindgen]
pub fn say_hello_from_rust() {
    log("Howdy!...from Rust and more.");

}

#[wasm_bindgen]
pub struct MyClient {

}

#[wasm_bindgen]
impl MyClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        log("MyClient created.");
        Self {}
    }

    pub fn update(&mut self, _time: f32, _height: f32, _width: f32) -> Result<(), JsValue> {
        //log("Update call.");
        Ok(())
    }

    pub fn render(&self) {
        log("Render call.");
    }
}