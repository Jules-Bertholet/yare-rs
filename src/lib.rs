//! # Rust bindings for [Yare.io](https://yare.io/) bots
//!
//! This crate uses [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) to expose the objects available to [Yare.io](https://yare.io/) bots to Rust.
//! It's meant to be used with [`yare-rust-template`](https://github.com/Jules-Bertholet/yare-rust-template),
//! which contains custom build scripts necessary to make `wasm-bindgen` work with Yare.
//!
//! ## Usage notes
//! The methods and structs this crate provides map pretty directly to what's available in JS.
//! This means they won't always be idiomatic Rust. For example, [`Deref`]-based inheritance is ued extensively;
//! this is [an antipattern](https://github.com/rust-unofficial/patterns/blob/master/anti_patterns/deref.md) for idiomatic Rust
//! but it's also the best/only way to represent JS inheritance hierarchies, and it's what `wasm-bindgen` uses.
//!
//! Passing values between WebAssembly and JS is slow, especially when those values aren't numbers.
//! Generally, any method in this crate that returns a value involves such a transfer of data
//! (functions that return static references don't).
//! So be careful, and only retrieve the information you need.
//!
//! For the reasons mentioned in the previous paragraphs, you may want to create you own structs and data structures to store the information you need.

use js_sys::{Array, Float64Array, JsString, Object, Reflect};
use std::{convert::TryFrom, ops::Deref, thread_local};
use wasm_bindgen::{prelude::*, JsCast};

/// The ID of any game entity, as reported by the [`id`](Entity::id) property, or by [`last_energized`](Entity::last_energized).
pub type EntityID = JsValue;

/// A position on the game board. Ordered pair of [`f64`].
pub type Position = [f64];

/// The [`hp`](Destructible::hp) of a destructible entity.
///
/// [Yare Documentation](https://yare.io/documentation)
#[wasm_bindgen]
pub enum HP {
    Inoperable = 0,
    Operable = 1,
}

/// The possible values of a spirit or base's [`shape`](Destructible::shape) property.
#[wasm_bindgen]
pub enum Shape {
    Circles = "circles",
    Squares = "squares",
    Triangles = "triangles",
}

/// The reason for which a spirit is inoperable.
/// If `Hostile`, then the spirit does not belong to you.
/// Otherwise, if `NoHP`, then the spirit has [`hp`](Spirit::hp) of 0 (dead or merged).
pub enum InoperableReason {
    Hostile,
    NoHP,
}

pub enum OperableSpiritShape<'a> {
    Circle(&'a OperableCircleSpirit),
    Square(&'a OperableSquareSpirit),
    Triangle(&'a OperableTriangleSpirit),
}

/// The possible [`structure_type`](Structure::structure_type)s.
///
/// [Yare Documentation](https://yare.io/documentation)
#[wasm_bindgen]
pub enum StructureType {
    Base = "base",
    Outpost = "outpost",
    Star = "star",
}

// Player
#[wasm_bindgen]
extern "C" {
    /// A player, as reported by the [`player_id`](Destructible::player_id) properties of spirits or bases,
    /// the [`control`](Outpost::control) property of the outpost, [`this_player_id`], or the property vales of [`players`].
    ///
    /// [Yare Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = JsString)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Player;
}

// OutpostSight
#[wasm_bindgen]
extern "C" {
    /// The [`sight`](Outpost::sight) of an outpost.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_outpost)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OutpostSight;

    #[wasm_bindgen(method, getter)]
    pub fn enemies(this: &OutpostSight) -> Vec<EntityID>;
}

// Sight
#[wasm_bindgen]
extern "C" {
    /// The [`sight`](Destructible::sight) of a spirit or base.
    ///
    /// [Yare Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = OutpostSight)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Sight;

    #[wasm_bindgen(method, getter)]
    pub fn friends(this: &Sight) -> Vec<EntityID>;

    #[wasm_bindgen(method, getter)]
    pub fn friends_beamable(this: &Sight) -> Vec<EntityID>;

    #[wasm_bindgen(method, getter)]
    pub fn enemies_beamable(this: &Sight) -> Vec<EntityID>;

    #[wasm_bindgen(method, getter)]
    pub fn structures(this: &Sight) -> Vec<EntityID>;
}

// Entity
#[wasm_bindgen]
extern "C" {
    /// Any object on the game board: can be a [`Spirit`], [`Base`], [`Outpost`], or [`Star`].
    ///
    /// [Yare Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Entity;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Entity) -> EntityID;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Entity) -> Box<Position>;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &Energizable) -> u16;

    #[wasm_bindgen(method, getter)]
    pub fn energy(this: &Energizable) -> u16;

    #[wasm_bindgen(method, getter)]
    pub fn last_energized(this: &Entity) -> EntityID;
}

// Energizable
#[wasm_bindgen]
extern "C" {
    /// Any [`Entity`] that is a valid target for an [`energize`](OperableSpirit::energize): can be a [`Spirit`], [`Base`], or [`Outpost`].
    ///
    /// [Yare Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = Entity)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Energizable;

    #[wasm_bindgen(method, getter)]
    pub fn energy_capacity(this: &Energizable) -> u16;
}

// Destructible
#[wasm_bindgen]
extern "C" {

    /// Any [`Entity`] that can be destroyed: can be a [`Spirit`] or [`Base`].
    ///
    /// [Yare Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = Energizable)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Destructible;

    #[wasm_bindgen(method, getter)]
    pub fn hp(this: &Destructible) -> HP;

    #[wasm_bindgen(method, getter)]
    pub fn sight(this: &Destructible) -> Sight;

    #[wasm_bindgen(method, getter)]
    pub fn player_id(this: &Destructible) -> Player;

    #[wasm_bindgen(method, getter)]
    pub fn shape(this: &Destructible) -> Shape;

    #[wasm_bindgen(method, getter)]
    pub fn color(this: &Destructible) -> String;
}

// Spirit
#[wasm_bindgen]
extern "C" {
    /// A spirit.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Destructible)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Spirit;

    #[wasm_bindgen(method, getter)]
    pub fn merged(this: &Spirit) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn move_speed(this: &Spirit) -> u16;

    #[wasm_bindgen(method, getter)]
    pub fn mark(this: &Spirit) -> String;
}

// OperableSpirit
#[wasm_bindgen]
extern "C" {
    /// A [`Spirit`] that is "operable", meaning that you can call methods on it.
    /// A spirit is "operable" if and only if it belongs to you, and it has an [`hp`](Destructible::hp) of 1.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSpirit;

    #[wasm_bindgen(method, js_name = "move")]
    pub fn r#move(this: &OperableSpirit, target: Array);

    #[wasm_bindgen(method)]
    pub fn energize(this: &OperableSpirit, target: Energizable);

    #[wasm_bindgen(method, js_name = "merge")]
    pub fn unchecked_merge(this: &OperableSpirit, target: &Spirit);

    #[wasm_bindgen(method, js_name = "divide")]
    pub fn unchecked_divide(this: &OperableSpirit);

    #[wasm_bindgen(method, js_name = "jump")]
    pub fn unchecked_jump(this: &OperableSquareSpirit, target: &Position);

    #[wasm_bindgen(method)]
    pub fn shout(this: &OperableSpirit, message: &str);

    #[wasm_bindgen(method)]
    pub fn set_mark(this: &OperableSpirit, label: &str);
}

impl OperableSpirit {
    pub fn move_(&self, pos: &Position) {
        let float_arr = Float64Array::new_with_length(2);
        float_arr.copy_from(pos);
        self.r#move(Array::from(float_arr.as_ref()));
    }
}

impl TryFrom<Spirit> for OperableSpirit {
    type Error = InoperableReason;

    fn try_from(s: Spirit) -> Result<Self, Self::Error> {
        if &s.player_id() == this_player_id() {
            if s.hp() as isize == 1 {
                return Ok(s.unchecked_into());
            } else {
                return Err(InoperableReason::NoHP);
            }
        } else {
            return Err(InoperableReason::Hostile);
        }
    }
}

impl<'a> TryFrom<&'a Spirit> for &'a OperableSpirit {
    type Error = InoperableReason;

    fn try_from(s: &'a Spirit) -> Result<Self, Self::Error> {
        if &s.player_id() == this_player_id() {
            if s.hp() as isize == 1 {
                return Ok(s.unchecked_ref());
            } else {
                return Err(InoperableReason::NoHP);
            }
        } else {
            return Err(InoperableReason::Hostile);
        }
    }
}

impl<'a> From<&'a OperableSpirit> for OperableSpiritShape<'a> {
    fn from(s: &'a OperableSpirit) -> OperableSpiritShape {
        return match s.shape() {
            Shape::Circles => OperableSpiritShape::Circle(s.unchecked_ref()),
            Shape::Squares => OperableSpiritShape::Square(s.unchecked_ref()),
            Shape::Triangles => OperableSpiritShape::Triangle(s.unchecked_ref()),
            _ => unreachable!("Unknown spirit type!"),
        };
    }
}

// OperableCircleSpirit
#[wasm_bindgen]
extern "C" {
    /// An [`OperableSpirit`] that is a circle. Can merge or divide.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = OperableSpirit)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableCircleSpirit;

    #[wasm_bindgen(method)]
    pub fn merge(this: &OperableCircleSpirit, target: &OperableCircleSpirit);

    #[wasm_bindgen(method)]
    pub fn divide(this: &OperableCircleSpirit);
}

// OperableSquareSpirit
#[wasm_bindgen]
extern "C" {
    /// An [`OperableSpirit`] that is a square. Can jump.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = OperableSpirit)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSquareSpirit;

    #[wasm_bindgen(method)]
    pub fn jump(this: &OperableSquareSpirit, target: &Position);
}

// OperableTriangleSpirit
#[wasm_bindgen]
extern "C" {
    /// An [`OperableSpirit`] that is a triangle.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableTriangleSpirit;
}

// Structure
#[wasm_bindgen]
extern "C" {
    /// A structure, i.e. anything with a [`structure_type`](Structure::structure_type): can be a [`Base`], [`Outpost`], or [`Star`].
    ///
    /// [Yare Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = Entity)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Structure;

    #[wasm_bindgen(method, getter)]
    pub fn structure_type(this: &Structure) -> StructureType;
}

// Base
#[wasm_bindgen]
extern "C" {
    /// A player base.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_base)
    #[wasm_bindgen(extends = Structure, extends = Destructible)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Base;

    #[wasm_bindgen(method, getter)]
    pub fn current_spirit_cost(this: &Base) -> u16;
}

// Outpost
#[wasm_bindgen]
extern "C" {
    /// An outpost.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_outpost)
    #[wasm_bindgen(extends = Structure, extends = Energizable)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Outpost;

    #[wasm_bindgen(method, getter)]
    pub fn range(this: &Outpost) -> u16;

    #[wasm_bindgen(method, getter)]
    pub fn sight(this: &Outpost) -> OutpostSight;

    #[wasm_bindgen(method, getter)]
    pub fn control(this: &Outpost) -> Player;
}

// Star
#[wasm_bindgen]
extern "C" {
    /// A star.
    ///
    /// [Yare Documentation](https://yare.io/documentation#doc_star)
    #[wasm_bindgen(extends = Structure)]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Star;
}

// GetById

/// This trait is implemented for the global objects that give mappings of [ID](EntityID)s to entities:
/// [`spirits`], [`bases`], [`outposts`], and [`stars`].
pub trait GetByID<T: JsCast>
where
    Self: AsRef<JsValue>,
    Self: Deref<Target = Object>,
{
    fn get(&self, id: &EntityID) -> Option<T> {
        return match Reflect::get(self.as_ref(), &id) {
            Ok(js_value) => Some(js_value.unchecked_into()),
            Err(_) => None,
        };
    }

    fn ids(&self) -> Vec<EntityID> {
        return Object::keys(self).to_vec();
    }

    fn values(&self) -> Vec<T> {
        return Object::values(self).iter().map(|jsval| T::unchecked_from_js(jsval)).collect();
    }
}

// `spirits`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Spirits;

    #[wasm_bindgen]
    static _spirits: Spirits;
}

impl GetByID<Spirit> for Spirits {}

/// `spirits`. Use the [`GetByID`] trait to retrieve individual spirits.
///
/// [Yare Documentation](https://yare.io/documentation#doc_spirit)
#[inline(always)]
pub fn spirits() -> &'static Spirits {
    return &_spirits;
}

// `bases`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Bases;

    #[wasm_bindgen]
    static _bases: Bases;
}

impl GetByID<Base> for Bases {}

/// `bases`. Use the [`GetByID`] trait to retrieve individual bases.
///
/// [Yare Documentation](https://yare.io/documentation#doc_base)
#[inline(always)]
pub fn bases() -> &'static Bases {
    return &_bases;
}

// `outposts`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Outposts;

    #[wasm_bindgen]
    static _outposts: Outposts;
}

impl GetByID<Outpost> for Outposts {}

/// `outposts`. Use the [`GetByID`] trait to retrieve individual outposts.
///
/// [Yare Documentation](https://yare.io/documentation#doc_outpost)
#[inline(always)]
pub fn outposts() -> &'static Outposts {
    return &_outposts;
}

// `stars`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Stars;

    #[wasm_bindgen]
    static _stars: Stars;
}

impl GetByID<Star> for Stars {}

/// `stars`. Use the [`GetByID`] trait to retrieve individual stars.
///
/// [Yare Documentation](https://yare.io/documentation#doc_star)
#[inline(always)]
pub fn stars() -> &'static Stars {
    return &_stars;
}

// `my_spirits`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static __my_spirits: Vec<EntityID>;
}

#[inline(always)]
fn _my_spirits() -> &'static Vec<EntityID> {
    return &__my_spirits;
}

// See JsStatic implementation
struct SpiritStatic<T: 'static> {
    pub __inner: &'static std::thread::LocalKey<T>,
}

trait Spirited {}

impl Spirited for Vec<&'static Spirit> {}

impl<T: Spirited + 'static> Deref for SpiritStatic<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.__inner.with(|ptr| &*(ptr as *const T)) }
    }
}

static MY_SPIRITS: SpiritStatic<Vec<&'static Spirit>> = {
    #[inline(always)]
    fn init() -> Vec<&'static Spirit> {
        return _my_spirits()
            .into_iter()
            .map(|jsval| Spirit::unchecked_from_js_ref(jsval))
            .collect();
    }
    thread_local!(
        static _VAL: Vec<&'static Spirit> = init();
    );
    SpiritStatic { __inner: &_VAL }
};

/// `my_spirits`, as a [`Vec`].
///
/// [Yare Documentation](https://yare.io/documentation#doc_spirit)
#[inline(always)]
pub fn my_spirits() -> &'static Vec<&'static Spirit> {
    return &MY_SPIRITS;
}

// `base`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _base: Base;
}

/// `base` (your base).
///
/// [Yare Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn base() -> &'static Base {
    return &_base;
}

// `enemy_base`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _enemy_base: Base;
}

/// `enemy_base` (the enemy base).
///
/// [Yare Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn enemy_base() -> &'static Base {
    return &_enemy_base;
}

// `outpost_mdo`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _outpost_mdo: Outpost;
}

/// `outpost_mdo`
///
/// [Yare Documentation](https://yare.io/documentation#doc_outpost)
#[inline(always)]
pub fn outpost_mdo() -> &'static Outpost {
    return &_outpost_mdo;
}

// `outpost`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _outpost: Outpost;
}

/// `outpost` (the outpost).
///
/// [Yare Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn outpost() -> &'static Outpost {
    return &_outpost;
}

// `star_zxq`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _star_zxq: Star;
}

/// `star_zxq` ([player 1](Players::p1)'s star).
///
/// [Yare Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn star_zxq() -> &'static Star {
    return &_star_zxq;
}

// `star_a1c`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _star_a1c: Star;
}

/// `star_a1c` ([player 2](Players::p2)'s star).
///
/// [Yare Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn star_a1c() -> &'static Star {
    return &_star_a1c;
}

// `star_p89`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _star_p89: Star;
}

/// `star_p89` (the outpost's star).
///
/// [Yare Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn star_p89() -> &'static Star {
    return &_star_p89;
}

// `this_player_id`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "this_player_id")]
    static _this_player_id: Player;
}

/// `this_player_id` (your player ID).
#[inline(always)]
pub fn this_player_id() -> &'static Player {
    return &_this_player_id;
}

// `players`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object)]
    #[derive(Clone, Debug)]
    pub type Players;

    #[wasm_bindgen(method, getter)]
    pub fn p1(this: &Players) -> Player;

    #[wasm_bindgen(method, getter)]
    pub fn p2(this: &Players) -> Player;

    #[wasm_bindgen]
    static _players: Players;
}

/// `players`. [`p1`](Players::p1) is the top-left player, [`p2`](Players::p2) is the bottom-right player.
#[inline(always)]
pub fn players() -> &'static Players {
    return &_players;
}

// `CODE_VERSION`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    static _CODE_VERSION: String;
}

/// `CODE_VERSION`
#[inline(always)]
pub fn CODE_VERSION() -> &'static String {
    return &_CODE_VERSION;
}
