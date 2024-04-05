use std::num::NonZeroU32;

use serde::{Deserialize, Serialize};

use crate::{
    character::SoulHack,
    lang::{Nameable, TextEntry, TextTable},
    IdInt, LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct EnemyRegistry {
    pub unique_monsters: Box<[UniqueMonster]>,
    pub enemies: Box<[Enemy]>,
}

#[derive(Serialize, Deserialize)]
pub struct EnemyLang {
    pub enemies: TextTable,
    pub enemy_groups: TextTable,
}

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    pub id: IdInt,
    pub name_id: IdInt,
}

#[derive(Serialize, Deserialize)]
pub struct UniqueMonster {
    pub id: IdInt,
    pub map_id: IdInt,
    pub name_id: IdInt,
    pub group_name: Option<IdInt>,
}

pub trait SoulLearnable {
    fn get_soul_hack(&self) -> Option<SoulHack>;
}

impl EnemyRegistry {
    pub fn get_enemy(&self, id: NonZeroU32) -> Option<&Enemy> {
        self.enemies.get(id.get() as usize - 1)
    }
}

impl Nameable for Enemy {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        language.enemies.enemies.get(self.name_id)
    }
}

impl Nameable for UniqueMonster {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        self.group_name
            .and_then(|i| language.enemies.enemy_groups.get(i))
            .or_else(|| language.enemies.enemies.get(self.name_id))
    }
}
