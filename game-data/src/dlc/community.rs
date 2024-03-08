use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, SortKey, TextEntry, TextTable},
    manual::Flag,
    npc::Npc,
    GameData, LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct DlcCommunity {
    npc_community: HashMap<u32, NpcCommunity>,
    pub npcs: Box<[Npc]>,
}

#[derive(Serialize, Deserialize)]
pub struct DlcCommunityLang {
    pub condition_lang: TextTable,
    pub npc_sort: SortKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CommunityTask {
    Talk {
        npc_id: u32,
        flag: Flag,
        max: u32,
    },
    Quest {
        quest_id: u32,
    },
    Condition {
        msg_id: u32,
        progress_flag: Option<Flag>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct NpcCommunity {
    pub progress_flag: Flag,
    pub order_flag: Flag,
    pub tasks: Box<[CommunityTask]>,
}

impl DlcCommunity {
    pub fn new(
        map: impl IntoIterator<Item = (u32, NpcCommunity)>,
        npcs: impl IntoIterator<Item = Npc>,
    ) -> Self {
        Self {
            npc_community: map.into_iter().collect(),
            npcs: npcs.into_iter().collect(),
        }
    }

    pub fn challenge(&self, npc_id: u32) -> &NpcCommunity {
        &self.npc_community[&npc_id]
    }

    pub fn npc_challenges(&self) -> impl Iterator<Item = (u32, &NpcCommunity)> {
        self.npc_community.iter().map(|(k, v)| (*k, v))
    }
}

impl CommunityTask {
    pub fn get_desc<'l>(&self, game: &GameData, lang: &'l LanguageData) -> Option<&'l str> {
        match self {
            CommunityTask::Talk { npc_id, .. } => {
                let name_id = game.npcs.get(*npc_id).name_id_hash;
                lang.npcs.get_npc_name(name_id).map(FilterEntry::text)
            }
            CommunityTask::Quest { quest_id } => {
                let name_id = game.quests.get(*quest_id as usize)?.name_id?;
                lang.quests.text.get(name_id).map(TextEntry::text)
            }
            CommunityTask::Condition { msg_id, .. } => lang
                .dlc
                .community
                .condition_lang
                .get(*msg_id as usize)
                .map(TextEntry::text),
        }
    }
}
