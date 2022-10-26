//! # Quest then module
//!
//! The 'then' quest is a quest that contains two sub-quests that
//! must be completed in order for the 'then' quest to be considered completed
//!
//! Progress on the second quest is not made until the first quest is completed

use std::fmt::Display;

use crate::{Event, Quest, QuestStatus};

/// The state of the then quest combinator
#[derive(Debug)]
pub struct QuestThen {
    /// The first quest to be completed
    q1: Box<dyn Quest<Event>>,
    /// The second quest to be completed
    q2: Box<dyn Quest<Event>>,
    /// The status of the then quest
    status: QuestStatus,
}

impl QuestThen {
    /// Create a new then quest given the two sub-quests to be completed in order
    pub fn new(q1: Box<dyn Quest<Event>>, q2: Box<dyn Quest<Event>>) -> Self {
        Self {
            q1,
            q2,
            status: QuestStatus::Ongoing,
        }
    }
}

impl Display for QuestThen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q1 = format!("{}", self.q1);
        let q1 = q1.replace('\n', "\n\t");
        let q2 = format!("{}", self.q2);
        let q2 = q2.replace('\n', "\n\t");
        match self.status {
            QuestStatus::Complete => write!(
                f,
                "[✅] You must, in order, complete each of these quests:\n\t{}\n\t{}",
                q1, q2
            ),
            QuestStatus::Ongoing => write!(
                f,
                "[ ] You must, in order, complete each of these quests:\n\t{}\n\t{}",
                q1, q2
            ),
        }
    }
}

impl Quest<Event> for QuestThen {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        if self.q1.register_event(event) == QuestStatus::Complete
            && self.q2.register_event(event) == QuestStatus::Complete
        {
            self.status = QuestStatus::Complete;
        }

        self.status
    }

    fn reset(&mut self) {
        self.q1.reset();
        self.q2.reset();
        self.status = QuestStatus::Ongoing;
    }
}
