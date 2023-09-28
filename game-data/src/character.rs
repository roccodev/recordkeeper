use std::num::NonZeroUsize;

use crate::{
    enemy::SoulLearnable,
    lang::{FilterEntry, FilterTable, Filterable, Id},
    LanguageData,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CharacterData {
    characters: Box<[Character]>,
    arts: Box<[Art]>,
    skills: Box<[Skill]>,
    classes: Box<[Class]>,
    attachments: Box<[Attachment]>,
    costumes: [Vec<Costume>; 6],
}

#[derive(Serialize, Deserialize)]
pub struct CharacterLang {
    pub characters: FilterTable,
    pub arts: FilterTable,
    pub skills: FilterTable,
    pub classes: FilterTable,
    pub misc: FilterTable,
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
    pub soul_hack: Option<SoulHack>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Skill {
    pub id: usize,
    pub name_id: usize,
    pub soul_hack: Option<SoulHack>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct SoulHack {
    pub status_flag: NonZeroUsize,
    pub achievement_flag: NonZeroUsize,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Class {
    pub id: usize,
    pub name_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Attachment {
    pub id: usize,
    pub name_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Costume {
    pub id: usize,
    pub name_id: usize,
}

impl CharacterData {
    pub fn new(
        characters: impl IntoIterator<Item = Character>,
        arts: impl IntoIterator<Item = Art>,
        skills: impl IntoIterator<Item = Skill>,
        classes: impl IntoIterator<Item = Class>,
        attachments: impl IntoIterator<Item = Attachment>,
        costumes: [Vec<Costume>; 6],
    ) -> Self {
        Self {
            characters: characters.into_iter().collect(),
            arts: arts.into_iter().collect(),
            skills: skills.into_iter().collect(),
            classes: classes.into_iter().collect(),
            attachments: attachments.into_iter().collect(),
            costumes,
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

    pub fn get_class(&self, id: usize) -> Option<&Class> {
        id.checked_sub(1).and_then(|id| self.classes.get(id))
    }

    pub fn characters(&self) -> &[Character] {
        &self.characters
    }

    pub fn classes(&self) -> &[Class] {
        &self.classes
    }

    pub fn arts(&self) -> &[Art] {
        &self.arts
    }

    pub fn skills(&self) -> &[Skill] {
        &self.skills
    }

    pub fn attachments(&self) -> &[Attachment] {
        &self.attachments
    }

    pub fn costumes(&self, char_id: usize) -> &[Costume] {
        char_id
            .checked_sub(1)
            .and_then(|i| self.costumes.get(i))
            .unwrap_or_else(|| &self.costumes[0])
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

impl Filterable for Class {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.characters.classes.get(self.name_id)
    }
}

impl Filterable for Attachment {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.characters.misc.get(self.name_id)
    }
}

impl Filterable for Costume {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.characters.misc.get(self.name_id)
    }
}

impl Id for Art {
    fn id(&self) -> usize {
        self.id
    }
}

impl Id for Skill {
    fn id(&self) -> usize {
        self.id
    }
}

impl Id for Class {
    fn id(&self) -> usize {
        self.id
    }
}

impl Id for Character {
    fn id(&self) -> usize {
        self.id
    }
}

impl Id for Attachment {
    fn id(&self) -> usize {
        self.id
    }
}

impl Id for Costume {
    fn id(&self) -> usize {
        self.id
    }
}

impl SoulLearnable for Art {
    fn get_soul_hack(&self) -> Option<SoulHack> {
        self.soul_hack
    }
}

impl SoulLearnable for Skill {
    fn get_soul_hack(&self) -> Option<SoulHack> {
        self.soul_hack
    }
}
