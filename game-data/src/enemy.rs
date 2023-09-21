use serde::{Deserialize, Serialize};

use crate::{
    lang::{Nameable, TextEntry, TextTable},
    LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct EnemyRegistry {
    pub unique_monsters: Box<[UniqueMonster]>,
}

#[derive(Serialize, Deserialize)]
pub struct EnemyLang {
    pub enemies: TextTable,
    pub enemy_groups: TextTable,
}

#[derive(Serialize, Deserialize)]
pub struct UniqueMonster {
    pub id: usize,
    pub map_id: usize,
    pub name_id: usize,
    pub group_name: Option<usize>,
}

impl Nameable for UniqueMonster {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        self.group_name
            .and_then(|i| language.enemies.enemy_groups.get(i))
            .or_else(|| language.enemies.enemies.get(self.name_id))
    }
}
