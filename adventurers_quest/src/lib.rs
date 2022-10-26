//! Game quest crate
//!
//! There are primitive quests as well as quest combinators. Quest combinators
//! can be used in a nested way to make very complicated quests.
//!
//! # Quickstart
//!
//! ```
//! let quest = Box::new(QuestMajority::new(
//!     Box::new(QuestThen::new(
//!         Box::new(QuestRepeat::new(Box::new(WalkQuest::new(Block::Sand)), 5)),
//!         Box::new(WalkQuest::new(Block::Object('x'))),
//!     )),
//!     Box::new(QuestThen::new(
//!         Box::new(WalkQuest::new(Block::Object('x'))),
//!         Box::new(WalkQuest::new(Block::Grass)),
//!     )),
//!     Box::new(QuestRepeat::new(
//!         Box::new(WalkRepeatQuest::new(Block::Water, 9)),
//!         2,
//!     )),
//! ));
//! ```

#![warn(missing_docs)]

use blocks::Block;

pub mod quests;

/// The status of a quest
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum QuestStatus {
    /// A complete quest
    Complete,

    /// An incomplete quest
    Ongoing,
}

/// This is what a "quest" should do.
/// Note that all `Quests` implement Debug and Display.
/// Traits' Debug implementation does not matter, but
/// they should implement [`std::fmt::Display`] to show
/// the current progress of the quest.
pub trait Quest<Event>: std::fmt::Display + std::fmt::Debug {
    /// Whenever something happens, you call "register_event" to tell the quest what's happened.
    ///
    /// Returns the updated status of the quest after the event has been processed
    fn register_event(&mut self, event: &Event) -> QuestStatus;

    /// Reset the quest, so that players can restart.
    fn reset(&mut self);
}

/// An event that contains various information that may affect the progress
/// of a quest
#[derive(Debug)]
pub struct Event {
    /// If some, the event indicates the movement to some block.
    block: Option<Block>,
}

impl Event {
    /// Create a new [`Event`] to indicate movement to a Block
    pub fn on_block(block: Block) -> Self {
        Self { block: Some(block) }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
