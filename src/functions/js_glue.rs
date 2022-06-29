use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js/dist/bundle_glue.js")]
extern "C" {
    pub async fn start_local_notification() -> JsValue;

}


// pub fn run() -> String {
//     format!("Hello from {}!", name())
// }
