use std::rc::Rc;

use crate::{
    lang::{FilterEntry, FilterTable, Filterable},
    LanguageData,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CharacterData {
    characters: Rc<[Character]>,
    arts: Rc<[Art]>,
    skills: Rc<[Skill]>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterLang {
    pub characters: FilterTable,
    pub arts: FilterTable,
    pub skills: FilterTable,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Character {
    pub id: usize,
    pub name_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Art {
    pub id: usize,
    pub name_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Skill {
    pub id: usize,
    pub name_id: usize,
}

impl CharacterData {
    pub fn new(
        characters: impl IntoIterator<Item = Character>,
        arts: impl IntoIterator<Item = Art>,
        skills: impl IntoIterator<Item = Skill>,
    ) -> Self {
        Self {
            characters: characters.into_iter().collect(),
            arts: arts.into_iter().collect(),
            skills: skills.into_iter().collect(),
        }
    }

    pub fn get_character(&self, id: usize) -> Option<&Character> {
        id.checked_sub(1).and_then(|id| self.characters.get(id))
    }

    pub fn get_art(&self, id: usize) -> Option<&Art> {
        id.checked_sub(1).and_then(|id| self.arts.get(id))
    }

    pub fn get_skill(&self, id: usize) -> Option<&Skill> {
        id.checked_sub(1).and_then(|id| self.skills.get(id))
    }
}

impl Filterable for Character {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.characters.characters.get(self.name_id)
    }
}

impl Filterable for Art {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.characters.arts.get(self.name_id)
    }
}

impl Filterable for Skill {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.characters.skills.get(self.name_id)
    }
}
