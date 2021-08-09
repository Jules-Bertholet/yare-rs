//! Provides access to spirits.

use crate::{
    prelude::*, CanFrom, Destructible, DestructibleID, LivingDestructible, LivingDestructibleID,
};
use js_sys::{Array, Float64Array, Object};
use std::convert::TryFrom;
use wasm_bindgen::prelude::*;

// Spirit
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`Spirit`].
    #[wasm_bindgen(extends = DestructibleID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type SpiritID;

    /// A spirit.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Destructible, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Spirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Spirit) -> SpiritID;

    #[wasm_bindgen(method, getter)]
    pub fn merged(this: &Spirit) -> Vec<DeadSpiritID>;

    #[wasm_bindgen(method, getter)]
    pub fn move_speed(this: &Spirit) -> f64;

    #[wasm_bindgen(method, getter)]
    pub fn mark(this: &Spirit) -> String;
}

impl CanFrom<Destructible> for Spirit {
    #[inline]
    fn can_from(value: &Destructible) -> bool {
        !<Base as CanFrom<Destructible>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<Destructible>, Error = Base for Spirit);

// FriendlySpirit
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`FriendlySpirit`].
    #[wasm_bindgen(extends = SpiritID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type FriendlySpiritID;

    /// A spirit that belongs to you.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type FriendlySpirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &FriendlySpirit) -> FriendlySpiritID;

    #[wasm_bindgen(method, getter)]
    pub fn merged(this: &FriendlySpirit) -> Vec<DeadFriendlySpiritID>;
}

impl CanFrom<Spirit> for FriendlySpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        &value.player_id() == this_player_id.as_ref()
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = EnemySpirit for FriendlySpirit);

// EnemySpirit
#[wasm_bindgen]
extern "C" {
    /// The ID of an [`EnemySpirit`].
    #[wasm_bindgen(extends = SpiritID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type EnemySpiritID;

    /// A spirit that does not belong to you.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type EnemySpirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &EnemySpirit) -> EnemySpiritID;

    #[wasm_bindgen(method, getter)]
    pub fn merged(this: &EnemySpirit) -> Vec<DeadEnemySpiritID>;
}

impl CanFrom<Spirit> for EnemySpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        !<FriendlySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = FriendlySpirit for EnemySpirit);

// LivingSpirit
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`LivingSpirit`].
    #[wasm_bindgen(extends = SpiritID, extends = LivingDestructibleID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type LivingSpiritID;

    /// A spirit that has positive HP.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit, extends = LivingDestructible, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type LivingSpirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &LivingSpirit) -> LivingSpiritID;
}

impl CanFrom<Spirit> for LivingSpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        value.hp() > 0
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = DeadSpirit for LivingSpirit);

impl CanFrom<LivingDestructible> for LivingSpirit {
    #[inline]
    fn can_from(value: &LivingDestructible) -> bool {
        !<Base as CanFrom<LivingDestructible>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<LivingDestructible>, Error = Base for LivingSpirit);

// DeadSpirit
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`DeadSpirit`].
    #[wasm_bindgen(extends = SpiritID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type DeadSpiritID;

    /// A spirit that has positive HP.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type DeadSpirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DeadSpirit) -> DeadSpiritID;
}

impl CanFrom<Spirit> for DeadSpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        !<LivingSpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = LivingSpirit for DeadSpirit);

// OperableSpirit
#[cfg(any(
    doc,
    not(any(feature = "circles", feature = "squares", feature = "triangles"))
))]
#[wasm_bindgen]
extern "C" {
    /// A [`Spirit`](crate::spirit::Spirit) that is "operable", meaning that you can call methods on it.
    /// A spirit is "operable" if and only if it belongs to you, and it has an [`hp`](Destructible::hp) of 1.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = FriendlySpirit, extends = LivingSpirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSpirit;
}

#[cfg(all(feature = "circles", not(doc)))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = FriendlySpirit, extends = LivingSpirit, typescript_type = "CircleSpirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSpirit;
}

#[cfg(all(feature = "squares", not(any(doc, feature = "circles"))))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = FriendlySpirit, extends = LivingSpirit, typescript_type = "SquareSpirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSpirit;
}

#[cfg(all(
    feature = "triangles",
    not(any(doc, feature = "circles", feature = "squares"))
))]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = FriendlySpirit, extends = LivingSpirit, typescript_type = "SquareSpirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSpirit;
}

#[wasm_bindgen]
extern "C" {
    /// The ID of an [`OperableSpirit`].
    #[wasm_bindgen(extends = FriendlySpiritID, extends = LivingSpiritID, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSpiritID;

    #[wasm_bindgen(method, js_name = "move")]
    fn r#move(this: &OperableSpirit, target: &Array);

    #[wasm_bindgen(method)]
    pub fn energize(this: &OperableSpirit, target: &Entity);

    /// Requires the `"circles"` crate feature
    #[cfg(feature = "circles")]
    #[wasm_bindgen(method)]
    pub fn merge(this: &OperableSpirit, target: &Spirit);

    /// Requires the `"circles"` crate feature
    #[cfg(feature = "circles")]
    #[wasm_bindgen(method)]
    pub fn divide(this: &OperableSpirit);

    /// Requires the `"squares"` crate feature
    #[cfg(feature = "squares")]
    #[wasm_bindgen(method, js_name = "jump")]
    fn _jump(this: &OperableSpirit, target: Array);

    /// Requires the `"triangles"` crate feature
    #[cfg(feature = "triangles")]
    #[wasm_bindgen(method)]
    pub fn explode(this: &OperableSpirit, target: Position);

    #[wasm_bindgen(method)]
    pub fn shout(this: &OperableSpirit, message: &str);

    #[wasm_bindgen(method)]
    pub fn set_mark(this: &OperableSpirit, label: &str);
}

pub type LivingFriendlySpirit = OperableSpirit;
pub type LivingFriendlySpiritID = OperableSpiritID;

impl OperableSpirit {
    /// `move` method
    pub fn move_to_pos(&self, pos: Position) {
        self.r#move(&pos.into());
    }

    #[cfg(feature = "squares")]
    pub fn jump(&self, pos: Position) {
        self._jump(pos.into());
    }
}

impl CanFrom<LivingSpirit> for LivingFriendlySpirit {
    #[inline]
    fn can_from(value: &LivingSpirit) -> bool {
        <FriendlySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<LivingSpirit>, Error = LivingEnemySpirit for LivingFriendlySpirit);

impl CanFrom<FriendlySpirit> for LivingFriendlySpirit {
    #[inline]
    fn can_from(value: &FriendlySpirit) -> bool {
        <LivingSpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<FriendlySpirit>, Error = DeadFriendlySpirit for LivingFriendlySpirit);

impl CanFrom<Spirit> for LivingFriendlySpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        <LivingSpirit as CanFrom<Spirit>>::can_from(value)
            && <FriendlySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = Spirit for LivingFriendlySpirit);

impl AsRef<Spirit> for LivingFriendlySpirit {
    #[inline]
    fn as_ref(&self) -> &Spirit {
        <LivingFriendlySpirit as AsRef<FriendlySpirit>>::as_ref(self).as_ref()
    }
}

// DeadFriendlySpirit
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`DeadFriendlySpirit`].
    #[wasm_bindgen(extends = FriendlySpiritID, extends = DeadSpiritID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type DeadFriendlySpiritID;

    /// A friendly spirit that has zero HP.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = FriendlySpirit, extends = DeadSpirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type DeadFriendlySpirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DeadFriendlySpirit) -> DeadFriendlySpiritID;
}

impl CanFrom<FriendlySpirit> for DeadFriendlySpirit {
    #[inline]
    fn can_from(value: &FriendlySpirit) -> bool {
        <DeadSpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<FriendlySpirit>, Error = LivingFriendlySpirit for DeadFriendlySpirit);

impl CanFrom<DeadSpirit> for DeadFriendlySpirit {
    #[inline]
    fn can_from(value: &DeadSpirit) -> bool {
        <FriendlySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<DeadSpirit>, Error = DeadEnemySpirit for DeadFriendlySpirit);

impl CanFrom<Spirit> for DeadFriendlySpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        <DeadSpirit as CanFrom<Spirit>>::can_from(value)
            && <FriendlySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = Spirit for DeadFriendlySpirit);

impl AsRef<Spirit> for DeadFriendlySpirit {
    #[inline]
    fn as_ref(&self) -> &Spirit {
        <DeadFriendlySpirit as AsRef<FriendlySpirit>>::as_ref(self).as_ref()
    }
}

// LivingEnemySpirit
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`LivingEnemySpirit`].
    #[wasm_bindgen(extends = EnemySpiritID, extends = LivingSpiritID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type LivingEnemySpiritID;

    /// An enemy spirit that has positive HP.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = EnemySpirit, extends = LivingSpirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type LivingEnemySpirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &LivingEnemySpirit) -> LivingEnemySpiritID;
}

impl CanFrom<EnemySpirit> for LivingEnemySpirit {
    #[inline]
    fn can_from(value: &EnemySpirit) -> bool {
        <LivingSpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<EnemySpirit>, Error = DeadEnemySpirit for LivingEnemySpirit);

impl CanFrom<LivingSpirit> for LivingEnemySpirit {
    #[inline]
    fn can_from(value: &LivingSpirit) -> bool {
        <EnemySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<LivingSpirit>, Error = LivingFriendlySpirit for LivingEnemySpirit);

impl CanFrom<Spirit> for LivingEnemySpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        <LivingSpirit as CanFrom<Spirit>>::can_from(value)
            && <EnemySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = Spirit for LivingEnemySpirit);

impl AsRef<Spirit> for LivingEnemySpirit {
    #[inline]
    fn as_ref(&self) -> &Spirit {
        <LivingEnemySpirit as AsRef<EnemySpirit>>::as_ref(self).as_ref()
    }
}

// DeadEnemySpirit
#[wasm_bindgen]
extern "C" {
    /// The ID of a [`DeadEnemySpirit`].
    #[wasm_bindgen(extends = EnemySpiritID, extends = DeadSpiritID, typescript_type = "SpiritID")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type DeadEnemySpiritID;

    /// An enemy spirit that has zero HP.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = EnemySpirit, extends = DeadSpirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type DeadEnemySpirit;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &DeadEnemySpirit) -> DeadEnemySpiritID;
}

impl CanFrom<EnemySpirit> for DeadEnemySpirit {
    #[inline]
    fn can_from(value: &EnemySpirit) -> bool {
        <DeadSpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<EnemySpirit>, Error = LivingEnemySpirit for DeadEnemySpirit);

impl CanFrom<DeadSpirit> for DeadEnemySpirit {
    #[inline]
    fn can_from(value: &DeadSpirit) -> bool {
        <EnemySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<DeadSpirit>, Error = DeadFriendlySpirit for DeadEnemySpirit);

impl CanFrom<Spirit> for DeadEnemySpirit {
    #[inline]
    fn can_from(value: &Spirit) -> bool {
        <DeadSpirit as CanFrom<Spirit>>::can_from(value)
            && <EnemySpirit as CanFrom<Spirit>>::can_from(value)
    }
}

try_can_from!(impl TryFrom<Spirit>, Error = Spirit for DeadEnemySpirit);

impl AsRef<Spirit> for DeadEnemySpirit {
    #[inline]
    fn as_ref(&self) -> &Spirit {
        <DeadEnemySpirit as AsRef<EnemySpirit>>::as_ref(self).as_ref()
    }
}

// `spirits`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof spirits)")]
    #[derive(Clone, Debug)]
    pub type Spirits;

    /// `spirits`. Use the [`GetByID`] trait to retrieve individual spirits.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen]
    pub static spirits: Spirits;
}

impl TryGetByID<EntityID, Spirit> for Spirits {}
impl TryGetByID<DestructibleID, Spirit> for Spirits {}
impl TryGetByID<LivingDestructibleID, LivingSpirit> for Spirits {}
impl GetByID<SpiritID, Spirit> for Spirits {}
impl EnumerateByID<SpiritID, Spirit> for Spirits {}
impl GetByID<LivingSpiritID, LivingSpirit> for Spirits {}
impl GetByID<DeadSpiritID, DeadSpirit> for Spirits {}
impl GetByID<FriendlySpiritID, FriendlySpirit> for Spirits {}
impl GetByID<EnemySpiritID, EnemySpirit> for Spirits {}
impl GetByID<OperableSpiritID, OperableSpirit> for Spirits {}
impl GetByID<DeadFriendlySpiritID, DeadFriendlySpirit> for Spirits {}
impl GetByID<LivingEnemySpiritID, LivingEnemySpirit> for Spirits {}
impl GetByID<DeadEnemySpiritID, DeadEnemySpirit> for Spirits {}

// `my_spirits`
#[wasm_bindgen]
extern "C" {
    /// `my_spirits`, as a [`Vec`].
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(method, getter)]
    pub static my_spirits: Vec<FriendlySpirit>;
}
