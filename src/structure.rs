//! Provides access to structures (bases, outposts, and stars).

use crate::{prelude::*, CanFrom, LivingEntity, LivingEntityID};
use js_sys::Reflect;
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

pub mod base;
pub mod outpost;
pub mod star;

/// The possible [`structure_type`](Structure::structure_type)s.
///
/// [Yare.io Documentation](https://yare.io/documentation)
#[wasm_bindgen(typescript_type = "StructureType")]
pub enum StructureType {
    Base = "base",
    Outpost = "outpost",
    Star = "star",
}

// Structure
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`Structure`].
    #[wasm_bindgen(extends = LivingEntityID, typescript_type = "StructureID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type StructureID;

    /// A structure, i.e. anything with a [`structure_type`](Structure::structure_type): can be a [`Base`], [`Outpost`], or [`Star`].
    ///
    /// [Yare.io Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = LivingEntity, typescript_type = "Structure")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Structure;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Structure) -> StructureID;

    #[wasm_bindgen(method, getter)]
    pub fn structure_type(this: &Structure) -> StructureType;

    #[wasm_bindgen(method, getter)]
    pub fn collision_radius(this: &Structure) -> f64;
}

impl CanFrom<Entity> for Structure {
    #[inline]
    fn can_from(value: &Entity) -> bool {
        Reflect::has(value, &"structure_type".into()).unwrap()
    }
}

try_can_from!(impl TryFrom<Entity>, Error = Entity for Structure);

impl CanFrom<LivingEntity> for Structure {
    #[inline]
    fn can_from(value: &LivingEntity) -> bool {
        <Structure as CanFrom<Entity>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<LivingEntity>, Error = Entity for Structure);
