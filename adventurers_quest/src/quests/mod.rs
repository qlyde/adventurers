//! The quests module
//!
//! Contains a series of primitive quests and combinator quests
//!
//! Primitive quests are simple quests that can be combined into a larger
//! quest using a combinator quest

pub mod combinators;
pub mod walk_quest;
pub mod walk_repeat_quest;

pub use walk_quest::*;
pub use walk_repeat_quest::*;
