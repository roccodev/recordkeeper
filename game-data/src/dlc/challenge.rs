use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, FilterTable, Filterable, Id, Nameable, TextEntry, TextTable},
    LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct ChallengeGame {
    pub challenges: Box<[ChallengeData]>,
    pub gauntlets: Box<[ChallengeData]>,
    pub emblems: Box<[Emblem]>,
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeLang {
    pub challenges: FilterTable,
    pub emblems: TextTable,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ChallengeData {
    pub id: usize,
    pub name_id: usize,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Emblem {
    pub id: usize,
    pub name_id: usize,
    pub levels: usize,
}

impl ChallengeGame {
    pub fn get_challenge(&self, id: usize) -> Option<&ChallengeData> {
        id.checked_sub(1).and_then(|idx| self.challenges.get(idx))
    }

    pub fn get_gauntlet(&self, id: usize) -> Option<&ChallengeData> {
        id.checked_sub(1).and_then(|idx| self.gauntlets.get(idx))
    }

    pub fn get_emblem(&self, id: usize) -> Option<&Emblem> {
        id.checked_sub(1).and_then(|idx| self.emblems.get(idx))
    }
}

impl Filterable for ChallengeData {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.dlc.challenge.challenges.get(self.name_id)
    }
}

impl Nameable for Emblem {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        language.dlc.challenge.emblems.get(self.name_id)
    }
}

impl Id for ChallengeData {
    fn id(&self) -> usize {
        self.id
    }
}
