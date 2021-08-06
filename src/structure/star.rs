//! Provides access to stars.

use crate::structure::{Structure, StructureID};
use crate::{prelude::*, CanFrom};
use js_sys::Object;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

// Star
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`Star`].
    #[wasm_bindgen(extends = StructureID, typescript_type = "StarID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type StarID;

    /// A star.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_star)
    #[wasm_bindgen(extends = Structure, typescript_type = "Star")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Star;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Star) -> StarID;

    #[wasm_bindgen(method, getter)]
    pub fn active_in(this: &Star) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn active_at(this: &Star) -> u32;
}

impl CanFrom<Structure> for Star {
    #[inline]
    fn can_from(value: &Structure) -> bool {
        value.structure_type().to_str() == "star"
    }
}

try_can_from!(impl TryFrom<Structure>, Error = Structure for Star);

// `stars`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof stars)")]
    #[derive(Clone, Debug)]
    pub type Stars;

    #[wasm_bindgen(js_name = "stars")]
    static _stars: Stars;
}

impl TryGetByID<EntityID, Star> for Stars {}
impl TryGetByID<StructureID, Star> for Stars {}
impl EnumerateByID<StarID, Star> for Stars {}
impl GetByID<StarID, Star> for Stars {}

/// `stars`. Use the [`GetByID`] trait to retrieve individual stars.
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_star)
#[inline(always)]
pub fn stars() -> &'static Stars {
    &_stars
}

// `star_zxq`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "star_zxq")]
    static _star_zxq: Star;
}

/// `star_zxq` ([player 1](crate::players::Players::p1)'s star).
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn star_zxq() -> &'static Star {
    &_star_zxq
}

// `star_a1c`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "star_a1c")]
    static _star_a1c: Star;
}

/// `star_a1c` ([player 2](crate::players::Players::p1)'s star).
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn star_a1c() -> &'static Star {
    &_star_a1c
}

// `star_p89`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "star_p89")]
    static _star_p89: Star;
}

/// `star_p89` (the outpost's star).
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn star_p89() -> &'static Star {
    &_star_p89
}
