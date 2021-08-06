//! Provides access to the built-in graphics methods.

use crate::prelude::*;
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "Graphics")]
    #[derive(Clone, Debug)]
    type Graphics;

    #[wasm_bindgen(method, getter)]
    fn style(this: &Graphics) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_style(this: &Graphics, style: &str);

    #[wasm_bindgen(method, getter)]
    fn linewidth(this: &Graphics) -> f64;

    #[wasm_bindgen(method, setter)]
    fn set_linewidth(this: &Graphics, linewidth: f64);

    #[wasm_bindgen(method)]
    fn line(this: &Graphics, pos: &Position, end: &Position);

    #[wasm_bindgen(method)]
    fn circle(this: &Graphics, pos: &Position, r: f64);

    #[wasm_bindgen(method)]
    fn rect(this: &Graphics, tl: &Position, br: &Position);

    #[wasm_bindgen]
    static graphics: Graphics;
}

#[inline(always)]
pub fn style() -> String {
    graphics.style()
}

#[inline(always)]
pub fn set_style(style: &str) {
    graphics.set_style(style);
}

#[inline(always)]
pub fn linewidth() -> f64 {
    graphics.linewidth()
}

#[inline(always)]
pub fn set_linewidth(linewidth: f64) {
    graphics.set_linewidth(linewidth);
}

#[inline(always)]
pub fn line(pos: &Position, end: &Position) {
    graphics.line(pos, end);
}

#[inline(always)]
pub fn circle(pos: &Position, r: f64) {
    graphics.circle(pos, r);
}

#[inline(always)]
pub fn rect(tl: &Position, br: &Position) {
    graphics.rect(tl, br);
}
