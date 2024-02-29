use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{lang::TextTable, manual::Flag};

#[derive(Serialize, Deserialize)]
pub struct DlcCommunity {
    npc_community: HashMap<usize, NpcCommunity>,
}

#[derive(Serialize, Deserialize)]
pub struct DlcCommunityLang {
    pub condition_lang: TextTable,
}

#[derive(Serialize, Deserialize)]
pub enum CommunityTask {
    Talk { npc_id: u32, flag: Flag, max: u32 },
    Quest { quest_id: u32 },
    Condition { msg_id: u32 },
}

#[derive(Serialize, Deserialize)]
pub struct NpcCommunity {
    pub progress_flag: Flag,
    pub order_flag: Flag,
    pub tasks: Box<[CommunityTask]>,
}

impl DlcCommunity {
    pub fn new(map: impl IntoIterator<Item = (usize, NpcCommunity)>) -> Self {
        Self {
            npc_community: map.into_iter().collect(),
        }
    }
}
