//! Quest IDs, flag IDs, conditions, etc.

use serde::{Deserialize, Serialize};
use strum::FromRepr;

use crate::lang::TextTable;

#[derive(Serialize, Deserialize)]
pub struct QuestRegistry {
    quests: Vec<Quest>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestLang {
    pub text: TextTable,
}

#[derive(Serialize, Deserialize)]
pub struct Quest {
    pub id: usize,
    pub name_id: Option<usize>,
    pub flag: usize,
    pub purposes: Vec<QuestPurpose>,
}

#[derive(Serialize, Deserialize)]
pub struct QuestPurpose {
    pub id: usize,
    pub flag: usize,
    pub tasks: [Option<PurposeTask>; 4],
}

#[derive(Serialize, Deserialize)]
pub struct PurposeTask {
    pub id: usize,
    pub name_id: Option<usize>,
    pub task_type: TaskType,
    pub flag: usize,
    pub branch: u8,
}

#[derive(Serialize, Deserialize, FromRepr)]
pub enum TaskType {
    Ask = 3,
    Battle = 0,
    Chase = 5,
    Collect = 7,
    Collepedia = 8,
    Condition = 11,
    Event = 2,
    Follow = 10,
    Gimmick = 9,
    Reach = 4,
    Request = 6,
    Talk = 1,
}

impl QuestRegistry {
    pub fn new(quests: Vec<Quest>) -> Self {
        Self { quests }
    }

    pub fn get(&self, id: usize) -> Option<&Quest> {
        id.checked_sub(1).and_then(|id| self.quests.get(id))
    }
}

impl QuestLang {
    pub fn new(table: TextTable) -> Self {
        Self { text: table }
    }
}
