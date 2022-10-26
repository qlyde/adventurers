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
    use super::*;
    use crate::quests::combinators::{QuestMajority, QuestRepeat, QuestThen};
    use crate::quests::{WalkQuest, WalkRepeatQuest};

    /// The player wins the game if they walk over 5 sand blocks
    #[test]
    fn q1_test() {
        let mut q1 = Box::new(QuestRepeat::new(Box::new(WalkQuest::new(Block::Sand)), 5));
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Cinderblock)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Flowerbush)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q1.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Complete
        );
    }

    /// "First, collect five objects called 'x'"
    /// "After finishing that, collect three objects called 'y'"
    #[test]
    fn q2_test() {
        let mut q2 = Box::new(QuestThen::new(
            Box::new(QuestRepeat::new(
                Box::new(WalkQuest::new(Block::Object('x'))),
                5,
            )),
            Box::new(QuestRepeat::new(
                Box::new(WalkQuest::new(Block::Object('y'))),
                3,
            )),
        ));
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q2.register_event(&Event::on_block(Block::Object('y'))),
            QuestStatus::Complete
        );
    }

    /// "The player wins the game if they do 2 of the following:"
    /// "walk over 5 blocks of sand", then "collect an 'x' object".
    /// "collect a 'y' object", then "walk on grass".
    /// "walk over 9 blocks of water, 3 times".
    #[test]
    fn q3_test1() {
        let mut q3 = Box::new(QuestMajority::new(
            Box::new(QuestThen::new(
                Box::new(QuestRepeat::new(Box::new(WalkQuest::new(Block::Sand)), 5)),
                Box::new(WalkQuest::new(Block::Object('x'))),
            )),
            Box::new(QuestThen::new(
                Box::new(WalkQuest::new(Block::Object('x'))),
                Box::new(WalkQuest::new(Block::Grass)),
            )),
            Box::new(QuestRepeat::new(
                Box::new(WalkRepeatQuest::new(Block::Water, 9)),
                2,
            )),
        ));
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Sand)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Complete
        );
    }

    #[test]
    fn q3_test2() {
        let mut q3 = Box::new(QuestMajority::new(
            Box::new(QuestThen::new(
                Box::new(QuestRepeat::new(Box::new(WalkQuest::new(Block::Sand)), 5)),
                Box::new(WalkQuest::new(Block::Object('x'))),
            )),
            Box::new(QuestThen::new(
                Box::new(WalkQuest::new(Block::Object('x'))),
                Box::new(WalkQuest::new(Block::Grass)),
            )),
            Box::new(QuestRepeat::new(
                Box::new(WalkRepeatQuest::new(Block::Water, 9)),
                2,
            )),
        ));
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Object('x'))),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Grass)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Ongoing
        );
        assert_eq!(
            q3.register_event(&Event::on_block(Block::Water)),
            QuestStatus::Complete
        );
    }
}
