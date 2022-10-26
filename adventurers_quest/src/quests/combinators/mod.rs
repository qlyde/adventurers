//! # Combinator quests
//!
//! Combinator quests are quests themselves that can be completed.
//! Their completion status depends on the completion status of their comprised quests.

pub mod quest_majority;
pub mod quest_repeat;
pub mod quest_then;

pub use quest_majority::*;
pub use quest_repeat::*;
pub use quest_then::*;
