//! # Quest repeat module
//!
//! The repeat quest is a quest that is completed when its sub-quest is completed
//! a certain amount of times

use std::fmt::Display;

use crate::{Event, Quest, QuestStatus};

/// The state of the repeat combinator quest
#[derive(Debug)]
pub struct QuestRepeat {
    /// The quest that must be repeated
    q: Box<dyn Quest<Event>>,
    /// The number of times the sub-quest must be completed
    /// for the combinator quest to be considered completed
    target_count: u32,
    /// The number of times the sub-quest has actually been completed
    completed_count: u32,
    /// The status of the repeat quest
    status: QuestStatus,
}

impl QuestRepeat {
    /// Create a new repeat quest given the quest to be repeated and the number of
    /// times to complete it in order for the combinator quest to be considered completed
    pub fn new(q: Box<dyn Quest<Event>>, target_count: u32) -> Self {
        Self {
            q,
            target_count,
            completed_count: 0,
            status: if target_count == 0 {
                QuestStatus::Complete
            } else {
                QuestStatus::Ongoing
            },
        }
    }
}

impl Display for QuestRepeat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q = format!("{}", self.q);
        let q = q.replace('\n', "\n\t");
        write!(
            f,
            "Repeat {} times:\n\t{}",
            self.target_count - self.completed_count,
            q
        )
    }
}

impl Quest<Event> for QuestRepeat {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        if self.status != QuestStatus::Complete
            && self.q.register_event(event) == QuestStatus::Complete
        {
            self.completed_count += 1;
            if self.completed_count == self.target_count {
                self.status = QuestStatus::Complete;
            } else {
                self.q.reset();
            }
        }

        self.status
    }

    fn reset(&mut self) {
        self.q.reset();
        self.completed_count = 0;
        self.status = QuestStatus::Ongoing;
    }
}
