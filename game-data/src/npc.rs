use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, FilterTable, Filterable, Id},
    LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct NpcRegistry {
    npcs: Box<[Npc]>,
}

#[derive(Serialize, Deserialize)]
pub struct NpcLang {
    pub npc_names: FilterTable,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Npc {
    pub id: u32,
    pub name_id_hash: u32,
}

impl NpcRegistry {
    pub fn new(iter: impl IntoIterator<Item = Npc>) -> Self {
        Self {
            npcs: iter.into_iter().collect(),
        }
    }

    pub fn get(&self, id: u32) -> &Npc {
        &self.npcs[self.npcs.binary_search_by_key(&id, |npc| npc.id).unwrap()]
    }
}

impl NpcLang {
    pub fn get_npc_name(&self, name_id_hash: u32) -> Option<&FilterEntry> {
        self.npc_names.get(name_id_hash as usize)
    }
}

impl Filterable for Npc {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.npcs.get_npc_name(self.name_id_hash)
    }
}

impl Id for Npc {
    fn id(&self) -> usize {
        self.id as usize
    }
}
