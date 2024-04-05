use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, FilterTable, Filterable, Id, Nameable, TextEntry, TextTable},
    IdInt, LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct ChallengeGame {
    pub challenges: Box<[ChallengeData]>,
    pub gauntlets: Box<[ChallengeData]>,
    pub emblems: Box<[Emblem]>,
    pub gauntlet_maps: Box<[GauntletMap]>,
    pub whimsy: Box<[Whimsy]>,
}

#[derive(Serialize, Deserialize)]
pub struct ChallengeLang {
    pub challenges: FilterTable,
    pub emblems: TextTable,
    pub whimsy: FilterTable,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct ChallengeData {
    pub id: u32,
    pub name_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Emblem {
    pub id: u32,
    pub name_id: u32,
    pub levels: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct GauntletMap {
    pub id: u32,
    pub based_on_lang_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Whimsy {
    pub id: u32,
    pub caption: u32,
}

impl ChallengeGame {
    pub fn get_challenge(&self, id: usize) -> Option<&ChallengeData> {
        id.checked_sub(1).and_then(|idx| self.challenges.get(idx))
    }

    pub fn get_gauntlet(&self, id: usize) -> Option<&ChallengeData> {
        id.checked_sub(1).and_then(|idx| self.gauntlets.get(idx))
    }

    pub fn get_emblem(&self, id: IdInt) -> Option<&Emblem> {
        id.checked_sub(1)
            .and_then(|idx| self.emblems.get(idx as usize))
    }
}

impl Filterable for ChallengeData {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.dlc.challenge.challenges.get(self.name_id)
    }
}

impl Filterable for GauntletMap {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.field.locations.get(self.based_on_lang_id)
    }
}

impl Filterable for Whimsy {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.dlc.challenge.whimsy.get(self.caption)
    }
}

impl Nameable for Emblem {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        language.dlc.challenge.emblems.get(self.name_id)
    }
}

impl Id for ChallengeData {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Id for GauntletMap {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Id for Whimsy {
    fn id(&self) -> u32 {
        self.id
    }
}
