//! Bindings relating to player IDs.

use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

// PlayerID
#[wasm_bindgen]
extern "C" {
    /// A player ID, as reported by the [`player_id`](Destructible::player_id) properties of spirits or bases,
    /// the [`control`](Outpost::control) property of the outpost, [`this_player_id`], or the property vales of [`players`].
    ///
    /// [Yare.io Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = JsString, typescript_type = "PlayerID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type PlayerID;
}

// `this_player_id`
#[wasm_bindgen]
extern "C" {
    /// `this_player_id` (your player ID).
    #[wasm_bindgen]
    pub static this_player_id: PlayerID;
}

// `players`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof players)")]
    #[derive(Clone, Debug)]
    pub type Players;

    #[wasm_bindgen(method, getter)]
    pub fn p1(this: &Players) -> PlayerID;

    #[wasm_bindgen(method, getter)]
    pub fn p2(this: &Players) -> PlayerID;

    /// `players`. [`p1`](Players::p1) is the top-left player, [`p2`](Players::p2) is the bottom-right player.
    #[wasm_bindgen]
    pub static players: Players;
}
