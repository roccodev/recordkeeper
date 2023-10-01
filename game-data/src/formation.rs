use serde::{Deserialize, Serialize};

use crate::{
    lang::{Nameable, TextEntry, TextTable},
    LanguageData,
};

#[derive(Serialize, Deserialize)]
pub struct FormationData {
    pub names: Box<[FormationNameProfile]>,
    pub colors: Box<[u32]>,
}

#[derive(Serialize, Deserialize)]
pub struct FormationLang {
    pub names: TextTable,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct FormationNameProfile {
    name: ProfileName,
    pub save_id: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ProfileName {
    Literal(usize),
    Challenge(usize),
}

impl FormationNameProfile {
    pub fn new(name: ProfileName, save_id: u16) -> Self {
        Self { name, save_id }
    }
}

impl Nameable for FormationNameProfile {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        match self.name {
            ProfileName::Literal(id) => language.formation.names.get(id),
            ProfileName::Challenge(id) => language.dlc.challenge.challenges.get(id).map(Into::into),
        }
    }
}
