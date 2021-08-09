//! Provides access to stars.

use crate::{prelude::*, CanFrom};
use crate::{Structure, StructureID};
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

    /// `stars`. Use the [`GetByID`] trait to retrieve individual stars.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_star)
    #[wasm_bindgen]
    pub static stars: Stars;
}

impl TryGetByID<EntityID, Star> for Stars {}
impl TryGetByID<StructureID, Star> for Stars {}
impl EnumerateByID<StarID, Star> for Stars {}
impl GetByID<StarID, Star> for Stars {}

// `star_zxq`
#[wasm_bindgen]
extern "C" {
    /// `star_zxq` ([player 1](crate::players::Players::p1)'s star).
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
    #[wasm_bindgen]
    pub static star_zxq: Star;
}

// `star_a1c`
#[wasm_bindgen]
extern "C" {
    /// `star_a1c` ([player 2](crate::players::Players::p1)'s star).
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
    #[wasm_bindgen]
    pub static star_a1c: Star;
}

// `star_p89`
#[wasm_bindgen]
extern "C" {
    /// `star_p89` (the outpost's star).
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
    #[wasm_bindgen]
    pub static star_p89: Star;
}
