//! Provides access to `console.log()`.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// `console.log`
    #[wasm_bindgen(js_name = "log", js_namespace = console, variadic)]
    pub fn log(args: Box<[JsValue]>);
}

/// Calls `console.log()` with the given comma-separated list of `Into<JsValue>` arguments.
#[macro_export]
macro_rules! log {
        ($($arg:expr),+) => {
            $crate::console::log(::std::boxed::Box::from([$(JsValue::from($arg),)+]));
        }
    }
