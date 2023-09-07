//! Quest IDs, flag IDs, conditions, etc.

use serde::{Deserialize, Serialize};
use strum::{EnumIter, FromRepr};

use crate::lang::{Nameable, TextTable};

#[derive(Serialize, Deserialize)]
pub struct QuestRegistry {
    quests: Vec<Quest>,
    dlc4_spacer: usize,
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

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct QuestPurpose {
    pub id: usize,
    pub flag: usize,
    pub tasks: [Option<PurposeTask>; 4],
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct PurposeTask {
    pub id: usize,
    pub name_id: Option<usize>,
    pub task_type: TaskType,
    pub flag: usize,
    pub branch: u8,
}

#[derive(Serialize, Deserialize, FromRepr, Clone, Copy, PartialEq, EnumIter)]
pub enum QuestStatus {
    Unstarted = 0,
    InProgress = 1,
    CompletedA = 2,
    CompletedB = 3,
}

#[derive(Serialize, Deserialize, FromRepr, Clone, Copy, PartialEq)]
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
    pub fn new(quests: Vec<Quest>, dlc4_spacer: usize) -> Self {
        Self {
            quests,
            dlc4_spacer,
        }
    }

    pub fn get(&self, id: usize) -> Option<&Quest> {
        id.checked_sub(1).and_then(|id| self.quests.get(id))
    }

    pub fn start(&self, dlc4: bool) -> usize {
        if dlc4 {
            self.dlc4_spacer.checked_add(1).unwrap()
        } else {
            1
        }
    }

    pub fn end(&self, dlc4: bool) -> usize {
        if dlc4 {
            self.quests.len()
        } else {
            self.dlc4_spacer.checked_sub(1).unwrap()
        }
    }
}

impl QuestLang {
    pub fn new(table: TextTable) -> Self {
        Self { text: table }
    }
}

impl TaskType {
    pub fn lang_id(&self) -> &str {
        match self {
            TaskType::Ask => "ask",
            TaskType::Battle => "battle",
            TaskType::Chase => "chase",
            TaskType::Collect => "collect",
            TaskType::Collepedia => "collepedia",
            TaskType::Condition => "condition",
            TaskType::Event => "event",
            TaskType::Follow => "follow",
            TaskType::Gimmick => "gimmick",
            TaskType::Reach => "reach",
            TaskType::Request => "request",
            TaskType::Talk => "talk",
        }
    }
}

impl Nameable for Quest {
    fn get_name<'l>(
        &self,
        language: &'l crate::LanguageData,
    ) -> Option<&'l crate::lang::TextEntry> {
        self.name_id.and_then(|id| language.quests.text.get(id))
    }
}

impl Nameable for PurposeTask {
    fn get_name<'l>(
        &self,
        language: &'l crate::LanguageData,
    ) -> Option<&'l crate::lang::TextEntry> {
        self.name_id.and_then(|id| language.quests.text.get(id))
    }
}
