///! Provides access to properties of the `memory` object.
///! Consider replacing with bindings that fit your own usage.
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "(typeof memory)")]
    type Memory;

    #[wasm_bindgen]
    static memory: Memory;

    #[wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &Memory, prop: &JsValue) -> JsValue;

    #[wasm_bindgen(method, structural, indexing_setter)]
    fn set(this: &Memory, prop: &JsValue, val: &JsValue);

    #[wasm_bindgen(method, structural, indexing_deleter)]
    fn delete(this: &Memory, prop: &JsValue);
}

/// Retrieve the value of `memory[prop]`.
#[inline(always)]
pub fn get(prop: &JsValue) -> JsValue {
    memory.get(prop)
}

/// Set the value of `memory[prop]`.
#[inline(always)]
pub fn set(prop: &JsValue, val: &JsValue) {
    memory.set(prop, val)
}

/// Delete `memory[prop]`.
#[inline(always)]
pub fn delete(prop: &JsValue) {
    memory.delete(prop)
}
