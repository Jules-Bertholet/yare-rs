//! Provides access to outposts.

use crate::players::PlayerID;
use crate::structure::{Structure, StructureID};
use crate::{prelude::*, CanFrom};
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

    #[wasm_bindgen(js_name = "outposts")]
    static _outposts: Outposts;
}

impl TryGetByID<EntityID, Outpost> for Outposts {}
impl TryGetByID<StructureID, Outpost> for Outposts {}
impl GetByID<OutpostID, Outpost> for Outposts {}
impl EnumerateByID<OutpostID, Outpost> for Outposts {}

/// `outposts`. Use the [`GetByID`] trait to retrieve individual outposts.
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
#[inline(always)]
pub fn outposts() -> &'static Outposts {
    &_outposts
}

// `outpost_mdo`
#[wasm_bindgen(js_name = "outpost_mdo")]
extern "C" {
    #[wasm_bindgen]
    static _outpost_mdo: Outpost;
}

/// `outpost_mdo`
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
#[inline(always)]
pub fn outpost_mdo() -> &'static Outpost {
    &_outpost_mdo
}

// `outpost`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "outpost")]
    static _outpost: Outpost;
}

/// `outpost` (the outpost).
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn outpost() -> &'static Outpost {
    &_outpost
}
