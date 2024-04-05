use std::num::NonZeroU32;

use crate::{
    dlc::pow_augment::PowAugment,
    enemy::SoulLearnable,
    lang::{FilterEntry, FilterTable, Filterable, Id},
    IdInt, LanguageData,
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

    pub pow_augment_characters: Box<[Character]>,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterLang {
    pub characters: FilterTable,
    pub arts: FilterTable,
    pub skills: FilterTable,
    pub classes: FilterTable,
    pub misc: FilterTable,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Character {
    pub id: IdInt,
    pub name_id: IdInt,
    pub pow_augment: Option<PowAugment>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Art {
    pub id: IdInt,
    pub name_id: IdInt,
    pub soul_hack: Option<SoulHack>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Skill {
    pub id: IdInt,
    pub name_id: IdInt,
    pub soul_hack: Option<SoulHack>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct SoulHack {
    pub status_flag: NonZeroU32,
    pub achievement_flag: NonZeroU32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Class {
    pub id: IdInt,
    pub name_id: IdInt,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Attachment {
    pub id: IdInt,
    pub name_id: IdInt,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Costume {
    pub id: IdInt,
    pub name_id: IdInt,
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
        let characters: Box<[Character]> = characters.into_iter().collect();
        Self {
            arts: arts.into_iter().collect(),
            skills: skills.into_iter().collect(),
            classes: classes.into_iter().collect(),
            attachments: attachments.into_iter().collect(),
            costumes,
            pow_augment_characters: characters
                .clone()
                .into_vec()
                .into_iter()
                .filter(|c| c.pow_augment.is_some())
                .collect(),
            characters,
        }
    }

    pub fn get_character(&self, id: IdInt) -> Option<&Character> {
        id.checked_sub(1)
            .and_then(|id| self.characters.get(id as usize))
    }

    pub fn get_art(&self, id: IdInt) -> Option<&Art> {
        id.checked_sub(1).and_then(|id| self.arts.get(id as usize))
    }

    pub fn get_skill(&self, id: IdInt) -> Option<&Skill> {
        id.checked_sub(1)
            .and_then(|id| self.skills.get(id as usize))
    }

    pub fn get_class(&self, id: IdInt) -> Option<&Class> {
        id.checked_sub(1)
            .and_then(|id| self.classes.get(id as usize))
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

    pub fn costumes(&self, char_id: IdInt) -> &[Costume] {
        char_id
            .checked_sub(1)
            .and_then(|i| self.costumes.get(i as usize))
            .unwrap_or_else(|| &self.costumes[0])
    }
}

impl Character {
    pub fn is_dlc4(&self) -> bool {
        [36, 37, 38, 39, 40, 41, 42, 43].contains(&self.id)
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
    fn id(&self) -> IdInt {
        self.id
    }
}

impl Id for Skill {
    fn id(&self) -> IdInt {
        self.id
    }
}

impl Id for Class {
    fn id(&self) -> IdInt {
        self.id
    }
}

impl Id for Character {
    fn id(&self) -> IdInt {
        self.id
    }
}

impl Id for Attachment {
    fn id(&self) -> IdInt {
        self.id
    }
}

impl Id for Costume {
    fn id(&self) -> IdInt {
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
