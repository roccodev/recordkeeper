use serde::{Deserialize, Serialize};

use crate::{
    lang::{FilterEntry, FilterTable, Filterable, Id},
    LanguageData,
};

#[derive(Serialize, Deserialize, PartialEq)]
pub struct FieldRegistry {
    maps: Box<[Map]>,
}

#[derive(Serialize, Deserialize)]
pub struct FieldLang {
    pub locations: FilterTable,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub struct Map {
    pub id: MapId,
    pub locations: Box<[Location]>,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct MapId {
    pub id: usize,
    pub name_id: usize,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct Location {
    pub id: usize,
    pub name_id: usize,
    pub location_type: LocationType,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum LocationType {
    Region,
    Location,
    Landmark,
    RestSpot,
    SecretArea,
    Colony,
    /// Invisible landmarks, usually only active for a
    /// specific story sequence.
    RespawnPoint,
}

impl FieldRegistry {
    pub fn new(maps: impl IntoIterator<Item = Map>) -> Self {
        Self {
            maps: maps.into_iter().collect(),
        }
    }

    pub fn get_map_by_id(&self, id: usize) -> Option<&Map> {
        id.checked_sub(1).and_then(|i| self.maps.get(i))
    }

    pub fn maps(&self) -> &[Map] {
        &self.maps
    }
}

impl Filterable for MapId {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.field.locations.get(self.name_id)
    }
}

impl Id for MapId {
    fn id(&self) -> usize {
        self.id
    }
}

impl Filterable for Map {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        self.id.get_filter(language)
    }
}

impl Id for Map {
    fn id(&self) -> usize {
        self.id.id()
    }
}

impl Filterable for Location {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry> {
        language.field.locations.get(self.name_id)
    }
}

impl Id for Location {
    fn id(&self) -> usize {
        self.id
    }
}
