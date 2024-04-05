use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::{
    enhance::Enhance,
    lang::{FilterEntry, FilterTable},
    GameData,
};

#[derive(Serialize, Deserialize)]
pub struct GameCraftItems {
    pub enhances: Vec<CraftEnhance>,
}

#[derive(Serialize, Deserialize)]
pub struct LangCraftItems {
    pub type_names: Rc<[CraftTypeText]>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CraftEnhance(pub [u32; 5]);

#[derive(Serialize, Deserialize, Clone)]
pub struct CraftTypeText {
    pub id: u32,
    pub text: Option<FilterEntry>,
}

impl LangCraftItems {
    pub fn new(table: FilterTable, id_to_lang: impl IntoIterator<Item = (u32, u32)>) -> Self {
        let type_names = id_to_lang
            .into_iter()
            .map(|(id, lang_id)| CraftTypeText {
                id,
                text: table.get(lang_id).cloned(),
            })
            .collect();
        Self { type_names }
    }

    pub fn index_of(&self, id: u32) -> Option<usize> {
        self.type_names.binary_search_by_key(&id, |t| t.id).ok()
    }
}

impl CraftEnhance {
    pub fn get_enhance_for_level<'g>(&self, game: &'g GameData, level: u32) -> Option<&'g Enhance> {
        game.enhance.get_instance(self.0[level as usize - 1])
    }
}
