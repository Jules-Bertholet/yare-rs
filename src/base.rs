//! Provides access to player bases.

use crate::spirit::LivingSpirit;
use crate::{
    prelude::*, CanFrom, Destructible, DestructibleID, LivingDestructible, LivingDestructibleID,
};
use crate::{Structure, StructureID};
use js_sys::{Object, Reflect};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

// Base
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`Base`](crate::structure::base::Base).
    #[wasm_bindgen(extends = StructureID, extends = LivingDestructibleID, typescript_type = "BaseID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type BaseID;

    /// A player base.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_base)
    #[wasm_bindgen(extends = Structure, extends = LivingDestructible, typescript_type = "Base")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Base;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Base) -> BaseID;

    #[wasm_bindgen(method, getter)]
    pub fn current_spirit_cost(this: &Base) -> i32;
}

impl CanFrom<Structure> for Base {
    #[inline]
    fn can_from(value: &Structure) -> bool {
        value.structure_type().to_str() == "base"
    }
}

try_can_from!(impl TryFrom<Structure>, Error = Structure for Base);

impl CanFrom<Destructible> for Base {
    #[inline]
    fn can_from(value: &Destructible) -> bool {
        Reflect::has(value, &"current_spirit_cost".into()).unwrap()
    }
}

try_can_from!(impl TryFrom<Destructible>, Error = Spirit for Base);

impl CanFrom<LivingDestructible> for Base {
    #[inline]
    fn can_from(value: &LivingDestructible) -> bool {
        <Base as CanFrom<Destructible>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<LivingDestructible>, Error = LivingSpirit for Base);

// `bases`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof bases)")]
    #[derive(Clone, Debug)]
    pub type Bases;

    /// `bases`. Use the [`GetByID`] trait to retrieve individual bases.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_base)
    #[wasm_bindgen]
    pub static bases: Bases;
}

impl TryGetByID<EntityID, Base> for Bases {}
impl TryGetByID<DestructibleID, Base> for Bases {}
impl TryGetByID<LivingDestructibleID, Base> for Bases {}
impl TryGetByID<StructureID, Base> for Bases {}
impl GetByID<BaseID, Base> for Bases {}
impl EnumerateByID<BaseID, Base> for Bases {}

// `base`
#[wasm_bindgen]
extern "C" {
    /// `base` (your base).
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
    #[wasm_bindgen]
    pub static base: Base;
}

// `enemy_base`
#[wasm_bindgen]
extern "C" {
    /// `enemy_base` (the enemy base).
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
    #[wasm_bindgen(js_name = "enemy_base")]
    pub static enemy_base: Base;
}
