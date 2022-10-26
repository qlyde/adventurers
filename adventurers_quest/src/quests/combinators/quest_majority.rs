//! # Quest majority module
//!
//! The majority quest is a quest that is completed when 2 out of its 3
//! sub-quests (i.e. the majority) are completed

use std::fmt::Display;

use crate::{Event, Quest, QuestStatus};

/// The state of the majority combinator quest
#[derive(Debug)]
pub struct QuestMajority {
    /// The first sub-quest
    q1: Box<dyn Quest<Event>>,
    /// The second sub-quest
    q2: Box<dyn Quest<Event>>,
    /// The third sub-quest
    q3: Box<dyn Quest<Event>>,
    /// Whether the first sub-quest is completed
    q1_complete: bool,
    /// Whether the second sub-quest is completed
    q2_complete: bool,
    /// Whether the third sub-quest is completed
    q3_complete: bool,
    /// The status of the majority quest
    status: QuestStatus,
}

impl QuestMajority {
    /// Create a new majority quest given three sub-quests
    ///
    /// All sub-quests start off as incomplete, and the majority quest itself
    /// starts off as ongoing
    pub fn new(
        q1: Box<dyn Quest<Event>>,
        q2: Box<dyn Quest<Event>>,
        q3: Box<dyn Quest<Event>>,
    ) -> Self {
        Self {
            q1,
            q2,
            q3,
            q1_complete: false,
            q2_complete: false,
            q3_complete: false,
            status: QuestStatus::Ongoing,
        }
    }
}

impl Display for QuestMajority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let q1 = format!("{}", self.q1);
        let q1 = q1.replace('\n', "\n\t");
        let q2 = format!("{}", self.q2);
        let q2 = q2.replace('\n', "\n\t");
        let q3 = format!("{}", self.q3);
        let q3 = q3.replace('\n', "\n\t");
        match self.status {
            QuestStatus::Complete => write!(
                f,
                "[✅] You must complete at least 2 of these quests:\n\t{}\n\t{}\n\t{}",
                q1, q2, q3
            ),
            QuestStatus::Ongoing => write!(
                f,
                "[ ] You must complete at least 2 of these quests:\n\t{}\n\t{}\n\t{}",
                q1, q2, q3
            ),
        }
    }
}

impl Quest<Event> for QuestMajority {
    fn register_event(&mut self, event: &Event) -> QuestStatus {
        // check if sub-quest 1 is complete
        if !self.q1_complete && self.q1.register_event(event) == QuestStatus::Complete {
            self.q1_complete = true;
        }

        // check if sub-quest 2 is complete
        if !self.q2_complete && self.q2.register_event(event) == QuestStatus::Complete {
            self.q2_complete = true;
        }

        // check if sub-quest 3 is complete
        if !self.q3_complete && self.q3.register_event(event) == QuestStatus::Complete {
            self.q3_complete = true;
        }

        // check if the majority of sub-quests are complete
        let completed_count = if self.q1_complete { 1 } else { 0 }
            + if self.q2_complete { 1 } else { 0 }
            + if self.q3_complete { 1 } else { 0 };

        if completed_count >= 2 {
            self.status = QuestStatus::Complete;
        }

        self.status
    }

    fn reset(&mut self) {
        self.q1.reset();
        self.q2.reset();
        self.q3.reset();
        self.q1_complete = false;
        self.q2_complete = false;
        self.q3_complete = false;
        self.status = QuestStatus::Ongoing;
    }
}
