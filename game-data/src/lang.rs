use crate::LanguageData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TextTable {
    entries: Vec<TextEntry>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TextEntry {
    text: Box<str>,
    text_lower: Box<str>,
    id: usize,
}

pub trait Nameable {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry>;

    fn get_name_str<'l>(&self, language: &'l LanguageData) -> Option<&'l str> {
        self.get_name(language).map(|t| t.text())
    }
}

impl TextTable {
    pub fn new(entries: impl IntoIterator<Item = TextEntry>) -> Self {
        let mut entries = entries.into_iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|e| e.id);
        Self { entries }
    }

    pub fn get(&self, id: usize) -> Option<&TextEntry> {
        self.entries
            .binary_search_by_key(&id, |e| e.id)
            .ok()
            .map(|i| &self.entries[i])
    }
}

impl TextEntry {
    pub fn new(text: &str, id: usize) -> Self {
        Self {
            text: Box::from(text),
            text_lower: Box::from(text.to_lowercase()),
            id,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn text_lower(&self) -> &str {
        &self.text_lower
    }
}
