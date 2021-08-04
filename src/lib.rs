#![doc = include_str!("../README.md")]

#[cfg(feature = "RenderService")]
pub mod render_service;

use js_sys::{global, Array, Float64Array, JsString, Object, Reflect};
use std::{convert::TryFrom, ops::Deref, thread_local};
use wasm_bindgen::{prelude::*, JsCast};

/// The ID of any game entity, as reported by the [`id`](Entity::id) property, or by [`last_energized`](Entity::last_energized).
pub type EntityID = JsValue;

/// A position on the game board. Ordered pair of [`f64`].
pub type Position = [f64];

/// The possible values of a spirit or base's [`shape`](Destructible::shape) property.
#[wasm_bindgen(typescript_type = "Shape")]
pub enum Shape {
    Circles = "circles",
    Squares = "squares",
    Triangles = "triangles",
}

/// The reason for which a spirit is inoperable.
/// If `Hostile`, then the spirit does not belong to you.
/// Otherwise, if `NoHP`, then the spirit has [`hp`](Destructible::hp) of 0 (dead or merged).
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
/// [Yare.io Documentation](https://yare.io/documentation)
#[wasm_bindgen(typescript_type = "StructureType")]
pub enum StructureType {
    Base = "base",
    Outpost = "outpost",
    Star = "star",
}

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

// OutpostSight
#[wasm_bindgen]
extern "C" {
    /// The [`sight`](Outpost::sight) of an outpost.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
    #[wasm_bindgen(extends = Object, typescript_type = "OutpostSight")]
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
    /// [Yare.io Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = OutpostSight, typescript_type = "Sight")]
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
    /// [Yare.io Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = Object, typescript_type = "Entity")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Entity;

    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Entity) -> EntityID;

    #[wasm_bindgen(method, getter)]
    pub fn position(this: &Entity) -> Box<Position>;

    #[wasm_bindgen(method, getter)]
    pub fn size(this: &Entity) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn energy(this: &Entity) -> i32;

    #[wasm_bindgen(method, getter)]
    pub fn last_energized(this: &Entity) -> EntityID;

    #[wasm_bindgen(method, getter)]
    pub fn energy_capacity(this: &Entity) -> i32;
}

// Destructible
#[wasm_bindgen]
extern "C" {
    /// Any [`Entity`] that can be destroyed: can be a [`Spirit`] or [`Base`].
    ///
    /// [Yare.io Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = Entity, typescript_type = "Destructible")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Destructible;

    #[wasm_bindgen(method, getter)]
    pub fn hp(this: &Destructible) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn sight(this: &Destructible) -> Sight;

    #[wasm_bindgen(method, getter)]
    pub fn player_id(this: &Destructible) -> PlayerID;

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
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Destructible, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Spirit;

    #[wasm_bindgen(method, getter)]
    pub fn merged(this: &Spirit) -> Array;

    #[wasm_bindgen(method, getter)]
    pub fn move_speed(this: &Spirit) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn mark(this: &Spirit) -> String;
}

// OperableSpirit
#[wasm_bindgen]
extern "C" {
    /// A [`Spirit`] that is "operable", meaning that you can call methods on it.
    /// A spirit is "operable" if and only if it belongs to you, and it has an [`hp`](Destructible::hp) of 1.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit, typescript_type = "Spirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableSpirit;

    #[wasm_bindgen(method, js_name = "move")]
    pub fn r#move(this: &OperableSpirit, target: Array);

    #[wasm_bindgen(method)]
    pub fn energize(this: &OperableSpirit, target: Entity);

    /// Like [`merge`](OperableCircleSpirit::merge), but without type-checks for shape.
    #[wasm_bindgen(method, js_name = "merge")]
    pub fn unchecked_merge(this: &OperableSpirit, target: &Spirit);

    /// Like [`divide`](OperableCircleSpirit::divide), but without type-checks for shape.
    #[wasm_bindgen(method, js_name = "divide")]
    pub fn unchecked_divide(this: &OperableSpirit);

    /// Like [`jump`](OperableSquareSpirit::jump), but without type-checks for shape.
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
            if s.hp() > 0 {
                Ok(s.unchecked_into())
            } else {
                Err(InoperableReason::NoHP)
            }
        } else {
            Err(InoperableReason::Hostile)
        }
    }
}

impl<'a> TryFrom<&'a Spirit> for &'a OperableSpirit {
    type Error = InoperableReason;

    fn try_from(s: &'a Spirit) -> Result<Self, Self::Error> {
        if &s.player_id() == this_player_id() {
            if s.hp() as isize >= 1 {
                return Ok(s.unchecked_ref());
            } else {
                Err(InoperableReason::NoHP)
            }
        } else {
            Err(InoperableReason::Hostile)
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
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = OperableSpirit, typescript_type = "CircleSpirit")]
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
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = OperableSpirit, typescript_type = "SquareSpirit")]
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
    /// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
    #[wasm_bindgen(extends = Spirit, typescript_type = "TriangleSpirit")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type OperableTriangleSpirit;

    #[wasm_bindgen(method)]
    pub fn explode(this: &OperableTriangleSpirit);
}

// Structure
#[wasm_bindgen]
extern "C" {
    /// A structure, i.e. anything with a [`structure_type`](Structure::structure_type): can be a [`Base`], [`Outpost`], or [`Star`].
    ///
    /// [Yare.io Documentation](https://yare.io/documentation)
    #[wasm_bindgen(extends = Entity, typescript_type = "Structure")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Structure;

    #[wasm_bindgen(method, getter)]
    pub fn structure_type(this: &Structure) -> StructureType;

    #[wasm_bindgen(method, getter)]
    pub fn collision_radius(this: &Structure) -> f64;
}

// Base
#[wasm_bindgen]
extern "C" {
    /// A player base.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_base)
    #[wasm_bindgen(extends = Structure, extends = Destructible, typescript_type = "Base")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Base;

    #[wasm_bindgen(method, getter)]
    pub fn current_spirit_cost(this: &Base) -> u32;
}

// Outpost
#[wasm_bindgen]
extern "C" {
    /// An outpost.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
    #[wasm_bindgen(extends = Structure, typescript_type = "Outpost")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Outpost;

    #[wasm_bindgen(method, getter)]
    pub fn range(this: &Outpost) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn sight(this: &Outpost) -> OutpostSight;

    #[wasm_bindgen(method, getter)]
    pub fn control(this: &Outpost) -> PlayerID;
}

// Star
#[wasm_bindgen]
extern "C" {
    /// A star.
    ///
    /// [Yare.io Documentation](https://yare.io/documentation#doc_star)
    #[wasm_bindgen(extends = Structure, typescript_type = "Star")]
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub type Star;

    #[wasm_bindgen(method, getter)]
    pub fn active_in(this: &Star) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn active_at(this: &Star) -> u32;
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
        return match Reflect::get(self.as_ref(), id) {
            Ok(js_value) => Some(js_value.unchecked_into()),
            Err(_) => None,
        };
    }

    fn ids(&self) -> Vec<EntityID> {
        Object::keys(self).to_vec()
    }

    fn values(&self) -> Vec<T> {
        return Object::values(self)
            .iter()
            .map(T::unchecked_from_js)
            .collect();
    }
}

// See JsStatic implementation
struct YareStatic<T: 'static> {
    pub __inner: &'static std::thread::LocalKey<T>,
}

trait YareStaticed {}

impl<T: YareStaticed + 'static> Deref for YareStatic<T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { self.__inner.with(|ptr| &*(ptr as *const T)) }
    }
}

// `spirits`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof spirits)")]
    #[derive(Clone, Debug)]
    pub type Spirits;

    #[wasm_bindgen(js_name = "spirits")]
    static _spirits: Spirits;
}

impl GetByID<Spirit> for Spirits {}

/// `spirits`. Use the [`GetByID`] trait to retrieve individual spirits.
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
#[inline(always)]
pub fn spirits() -> &'static Spirits {
    &_spirits
}

// `bases`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof bases)")]
    #[derive(Clone, Debug)]
    pub type Bases;

    #[wasm_bindgen(js_name = "bases")]
    static _bases: Bases;
}

impl GetByID<Base> for Bases {}

/// `bases`. Use the [`GetByID`] trait to retrieve individual bases.
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_base)
#[inline(always)]
pub fn bases() -> &'static Bases {
    &_bases
}

// `outposts`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof outposts)")]
    #[derive(Clone, Debug)]
    pub type Outposts;

    #[wasm_bindgen(js_name = "outposts")]
    static _outposts: Outposts;
}

impl GetByID<Outpost> for Outposts {}

/// `outposts`. Use the [`GetByID`] trait to retrieve individual outposts.
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_outpost)
#[inline(always)]
pub fn outposts() -> &'static Outposts {
    &_outposts
}

// `stars`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(extends = Object, typescript_type = "(typeof stars)")]
    #[derive(Clone, Debug)]
    pub type Stars;

    #[wasm_bindgen(js_name = "stars")]
    static _stars: Stars;
}

impl GetByID<Star> for Stars {}

/// `stars`. Use the [`GetByID`] trait to retrieve individual stars.
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_star)
#[inline(always)]
pub fn stars() -> &'static Stars {
    &_stars
}

// `my_spirits`
impl YareStaticed for Vec<Spirit> {}

#[allow(bad_style)]
#[allow(clippy::all)]
static _my_spirits: YareStatic<Vec<Spirit>> = {
    #[inline(always)]
    fn init() -> Vec<Spirit> {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen]
            #[derive(Clone, Debug)]
            type GlobalThis;

            #[wasm_bindgen(method, getter)]
            fn my_spirits(this: &GlobalThis) -> Vec<JsValue>;
        }

        return global()
            .unchecked_into::<GlobalThis>()
            .my_spirits()
            .drain(..)
            .map(|js_value| Spirit::unchecked_from_js(js_value))
            .collect();
    }
    thread_local!(
        static _VAL: Vec<Spirit> = init();
    );
    YareStatic { __inner: &_VAL }
};

/// `my_spirits`, as a [`Vec`].
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_spirit)
#[inline(always)]
pub fn my_spirits() -> &'static Vec<Spirit> {
    &_my_spirits
}

// `base`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "base")]
    static _base: Base;
}

/// `base` (your base).
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn base() -> &'static Base {
    &_base
}

// `enemy_base`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "enemy_base")]
    static _enemy_base: Base;
}

/// `enemy_base` (the enemy base).
///
/// [Yare.io Documentation](https://yare.io/documentation#doc_intro)
#[inline(always)]
pub fn enemy_base() -> &'static Base {
    &_enemy_base
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

// `star_zxq`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "star_zxq")]
    static _star_zxq: Star;
}

/// `star_zxq` ([player 1](Players::p1)'s star).
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

/// `star_a1c` ([player 2](Players::p2)'s star).
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

// `this_player_id`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "this_player_id")]
    static _this_player_id: PlayerID;
}

/// `this_player_id` (your player ID).
#[inline(always)]
pub fn this_player_id() -> &'static PlayerID {
    &_this_player_id
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

    #[wasm_bindgen(js_name = "players")]
    static _players: Players;
}

/// `players`. [`p1`](Players::p1) is the top-left player, [`p2`](Players::p2) is the bottom-right player.
#[inline(always)]
pub fn players() -> &'static Players {
    &_players
}

// `tick`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "tick")]
    static _tick: u32;
}

/// `tick` (the number of ticks since the start of the game).
#[inline(always)]
pub fn tick() -> &'static u32 {
    &_tick
}

// `graphics`
/// Module for Yare's built-in graphics methods.
pub mod graphics {
    use crate::Position;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = graphics, getter)]
        pub fn style() -> String;

        #[wasm_bindgen(js_namespace = graphics, setter)]
        pub fn set_style(style: &str);

        #[wasm_bindgen(js_namespace = graphics, getter)]
        pub fn linewidth() -> f64;

        #[wasm_bindgen(js_namespace = graphics, setter)]
        pub fn set_linewidth(linewidth: f64);

        #[wasm_bindgen(js_namespace = graphics)]
        pub fn line(pos: &Position, end: &Position);

        #[wasm_bindgen(js_namespace = graphics)]
        pub fn circle(pos: &Position, r: f64);

        #[wasm_bindgen(js_namespace = graphics)]
        pub fn rect(tl: &Position, br: &Position);
    }
}

/// `console.log`
pub mod console {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        /// `console.log`
        #[wasm_bindgen(js_name = "log", js_namespace = console, variadic)]
        pub fn log(args: Box<[JsValue]>);
    }

    /// Calls `console.log()`
    #[macro_export]
    macro_rules! log {
        ($($arg:expr),+) => {
            $crate::console::log(::std::boxed::Box::from([$(JsValue::from($arg),)+]));
        }
    }
}

// `CODE_VERSION`
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "CODE_VERSION")]
    static _CODE_VERSION: String;
}

/// `CODE_VERSION`
#[allow(bad_style)]
#[allow(clippy::all)]
#[inline(always)]
pub fn CODE_VERSION() -> &'static String {
    return &_CODE_VERSION;
}

// `no_entity`
impl YareStaticed for EntityID {}

#[allow(bad_style)]
#[allow(clippy::all)]
static _NULl_ENTITY_ID: YareStatic<EntityID> = {
    #[inline(always)]
    fn init() -> EntityID {
        return "".into();
    }
    thread_local!(
        static _VAL: EntityID = init();
    );
    YareStatic { __inner: &_VAL }
};

/// Represents the EntityID for when there is no entity.
/// Just an empty JS string.
#[inline(always)]
pub fn no_entity() -> &'static EntityID {
    &_NULl_ENTITY_ID
}

// `memory`
/// Module for accessing properties of the `memory` object.
/// Consider replacing with bindings that fit your own usage.
pub mod memory {
    use wasm_bindgen::{JsStatic, prelude::*};

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen]
        type Memory;

        #[wasm_bindgen]
        static memory: Memory;

        #[wasm_bindgen(method, structural, indexing_getter)]
        fn get(this: &Memory, prop: &JsValue) -> JsValue;

        #[wasm_bindgen(method, structural, indexing_setter)]
        fn set(this: &Memory, prop: &JsValue, val: &JsValue);

        #[wasm_bindgen(method, structural, indexing_deleter)]
        fn delete(this: &Memory, prop: &JsValue);
    }

    /// Retrieve the value of `memory[prop]`.
    #[inline(always)]
    pub fn get(prop: &JsValue) -> JsValue {
        memory.get(prop)
    }

    /// Set the value of `memory[prop]`.
    #[inline(always)]
    pub fn set(prop: &JsValue, val: &JsValue) {
        memory.set(prop, val)
    }

    /// Delete `memory[prop]`.
    #[inline(always)]
    pub fn delete(prop: &JsValue) {
        memory.delete(prop)
    }
}
