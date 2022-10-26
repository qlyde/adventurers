//! # The walk quest module
//!
//! The walk quest is a primitive quest that is quite simple. The quest is given
//! a target block and is completed once an [`Event`] is fired that indicates
//! movement onto this target block
//!
//! In other words, if the target block is reached, the walk quest is complete

use std::fmt::Display;

use blocks::Block;

use crate::{Event, Quest, QuestStatus};

/// The state of the walk primitive quest
#[derive(Debug)]
pub struct WalkQuest {
    /// The target block to walk to
    target_block: Block,
    /// The status of the walk quest
    status: QuestStatus,
}

impl WalkQuest {
    /// Create a new walk quest given the target block
    ///
    /// The quest starts as ongoing
    pub fn new(target_block: Block) -> Self {
        Self {
            target_block,
            status: QuestStatus::Ongoing,
        }
    }
}

impl Display for WalkQuest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            QuestStatus::Complete => write!(f, "[✅] Walk on a {} block", self.target_block),
            QuestStatus::Ongoing => write!(f, "[ ] Walk on a {} block", self.target_block),
        }
    }
}

impl Quest<Event> for WalkQuest {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        if self.status != QuestStatus::Complete {
            if let Some(block) = &event.block {
                if *block == self.target_block {
                    self.status = QuestStatus::Complete;
                }
            }
        }

        self.status
    }

    fn reset(&mut self) {
        self.status = QuestStatus::Ongoing;
    }
}
