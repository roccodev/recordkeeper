use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, FilterTable, Filterable, Id},
    IdInt, LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct FormationData {
    pub names: Box<[FormationNameProfile]>,
    pub colors: Box<[u32]>,
}

#[derive(Serialize, Deserialize)]
pub struct FormationLang {
    pub names: FilterTable,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FormationNameProfile {
    name: ProfileName,
    pub save_id: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ProfileName {
    Literal(IdInt),
    Challenge(IdInt),
}

impl FormationNameProfile {
    pub fn new(name: ProfileName, save_id: u16) -> Self {
        Self { name, save_id }
    }
}

impl Filterable for FormationNameProfile {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        match self.name {
            ProfileName::Literal(id) => language.formation.names.get(id),
            ProfileName::Challenge(id) => language.dlc.challenge.challenges.get(id),
        }
    }
}

impl Id for FormationNameProfile {
    fn id(&self) -> IdInt {
        self.save_id.into()
    }
}
