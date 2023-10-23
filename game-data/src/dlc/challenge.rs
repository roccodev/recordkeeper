use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, FilterTable, Filterable},
    LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct ChallengeGame {
    pub challenges: Box<[ChallengeData]>,
    pub gauntlets: Box<[ChallengeData]>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ChallengeData {
    pub id: usize,
    pub name_id: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeLang {
    pub challenges: FilterTable,
}

impl ChallengeGame {
    pub fn get_challenge(&self, id: usize) -> Option<&ChallengeData> {
        id.checked_sub(1).and_then(|idx| self.challenges.get(idx))
    }

    pub fn get_gauntlet(&self, id: usize) -> Option<&ChallengeData> {
        id.checked_sub(1).and_then(|idx| self.gauntlets.get(idx))
    }
}

impl Filterable for ChallengeData {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.dlc.challenge.challenges.get(self.name_id)
    }
}
