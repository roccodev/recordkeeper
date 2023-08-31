use serde::{Deserialize, Serialize};

use crate::{
    enhance::Enhance,
    lang::{TextEntry, TextTable},
    GameData,
};

#[derive(Serialize, Deserialize)]
pub struct GameCraftItems {
    pub enhances: Vec<CraftEnhance>,
}

#[derive(Serialize, Deserialize)]
pub struct LangCraftItems {
    type_names: Vec<CraftTypeText>,
}

#[derive(Serialize, Deserialize)]
pub struct CraftEnhance(pub [u32; 5]);

#[derive(Serialize, Deserialize)]
struct CraftTypeText {
    id: u32,
    text: Option<TextEntry>,
}

impl LangCraftItems {
    pub fn new(table: TextTable, id_to_lang: impl IntoIterator<Item = (u32, u32)>) -> Self {
        let type_names = id_to_lang
            .into_iter()
            .map(|(id, lang_id)| CraftTypeText {
                id,
                text: table.get(lang_id as usize).cloned(),
            })
            .collect();
        Self { type_names }
    }
}

impl CraftEnhance {
    pub fn get_enhance_for_level<'g>(&self, game: &'g GameData, level: u32) -> Option<&'g Enhance> {
        game.enhance.get_instance(self.0[level as usize] as u32)
    }
}
