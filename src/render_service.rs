//! Provides bindings for `RenderService` from [`yare-code-sync`](https://github.com/arikwex/yare-code-sync).
//! To use this module, you will need to enable the crate's `RenderService` feature.

use crate::Position;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "yare-code-sync/client/RenderService")]
extern "C" {
    #[wasm_bindgen]
    pub fn circle(pos: Position, radius: f64, color: &str);

    #[wasm_bindgen(js_name = "circle")]
    pub fn circle_color(pos: Position, radius: f64, color: &str);

    #[wasm_bindgen]
    pub fn ping(pos: Position);

    #[wasm_bindgen]
    pub fn line(pos1: Position, pos2: Position);

    #[wasm_bindgen(js_name = "line")]
    pub fn line_color(pos1: Position, pos2: Position, color: &str);

    #[wasm_bindgen]
    pub fn text(pos: Position, str: &str);

    #[wasm_bindgen(js_name = "text")]
    pub fn text_color(pos: Position, str: &str, color: &str);

    #[wasm_bindgen]
    pub fn log(str: &str);

    #[wasm_bindgen(js_name = "log")]
    pub fn log_js(obj: &JsValue);
}
