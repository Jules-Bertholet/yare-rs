//! Provides access to outposts.

use crate::players::PlayerID;
use crate::{prelude::*, CanFrom};
use crate::{Structure, StructureID};
use js_sys::Object;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

// Outpost
#[wasm_bindgen]
extern "C" {
    /// The ID of an [`Outpost`].
    #[wasm_bindgen(extends = StructureID, typescript_type = "OutpostID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OutpostID;

    /// An outpost.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
    #[wasm_bindgen(extends = Structure, typescript_type = "Outpost")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Outpost;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Outpost) -> OutpostID;

    #[wasm_bindgen(method, getter)]
    pub fn range(this: &Outpost) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn sight(this: &Outpost) -> OutpostSight;

    #[wasm_bindgen(method, getter)]
    pub fn control(this: &Outpost) -> PlayerID;
}

impl CanFrom<Structure> for Outpost {
    #[inline]
    fn can_from(value: &Structure) -> bool {
        value.structure_type().to_str() == "outpost"
    }
}

try_can_from!(impl TryFrom<Structure>, Error = Structure for Outpost);

// `outposts`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof outposts)")]
    #[derive(Clone, Debug)]
    pub type Outposts;

    /// `outposts`. Use the [`GetByID`] trait to retrieve individual outposts.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
    #[wasm_bindgen]
    pub static outposts: Outposts;
}

impl TryGetByID<EntityID, Outpost> for Outposts {}
impl TryGetByID<StructureID, Outpost> for Outposts {}
impl GetByID<OutpostID, Outpost> for Outposts {}
impl EnumerateByID<OutpostID, Outpost> for Outposts {}

// `outpost_mdo`
#[wasm_bindgen(js_name = "outpost_mdo")]
extern "C" {
    /// `outpost_mdo`
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
    #[wasm_bindgen]
    pub static outpost_mdo: Outpost;
}

// `outpost`
#[wasm_bindgen]
extern "C" {
    /// `outpost` (the outpost).
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
    #[wasm_bindgen]
    pub static outpost: Outpost;
}
