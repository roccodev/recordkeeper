use crate::LanguageData;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
pub struct TextTable {
    entries: Vec<TextEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct TextEntry {
    text: Box<str>,
    id: usize,
}

pub trait Nameable {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l str>;
}

impl TextTable {
    pub fn new(entries: impl IntoIterator<Item = TextEntry>) -> Self {
        let mut entries = entries.into_iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|e| e.id);
        Self { entries }
    }

    pub fn get(&self, id: usize) -> Option<&str> {
        self.entries
            .binary_search_by_key(&id, |e| e.id)
            .ok()
            .map(|i| self.entries[i].text.as_ref())
    }
}

impl TextEntry {
    pub fn new(text: &str, id: usize) -> Self {
        Self {
            text: Box::from(text),
            id,
        }
    }
}
