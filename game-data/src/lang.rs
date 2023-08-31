use crate::LanguageData;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Table<T> {
    entries: Vec<T>,
}

pub type TextTable = Table<TextEntry>;
pub type FilterTable = Table<FilterEntry>;

#[derive(Serialize, Deserialize, Clone)]
pub struct TextEntry {
    text: Box<str>,
    id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FilterEntry {
    text: TextEntry,
    text_lower: Box<str>,
}

pub trait Nameable {
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry>;

    fn get_name_str<'l>(&self, language: &'l LanguageData) -> Option<&'l str> {
        self.get_name(language).map(|t| t.text())
    }
}

pub trait Filterable {
    fn get_filter<'l>(&self, language: &'l LanguageData) -> Option<&'l FilterEntry>;
}

pub trait Id {
    fn id(&self) -> usize;
}

impl<T> Table<T>
where
    T: Id,
{
    pub fn new(entries: impl IntoIterator<Item = T>) -> Self {
        let mut entries = entries.into_iter().collect::<Vec<_>>();
        entries.sort_unstable_by_key(|e| e.id());
        Self { entries }
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.entries
            .binary_search_by_key(&id, |e| e.id())
            .ok()
            .map(|i| &self.entries[i])
    }
}

impl TextEntry {
    pub fn new(text: &str, id: usize) -> Self {
        Self {
            text: Box::from(text),
            id,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl FilterEntry {
    pub fn new(text: &str, id: usize) -> Self {
        TextEntry::new(text, id).into()
    }

    pub fn text(&self) -> &str {
        self.text.text()
    }

    pub fn text_lower(&self) -> &str {
        &self.text_lower
    }
}

impl<T> Nameable for T
where
    T: Filterable,
{
    fn get_name<'l>(&self, language: &'l LanguageData) -> Option<&'l TextEntry> {
        self.get_filter(language).map(|f| &f.text)
    }
}

impl Id for TextEntry {
    fn id(&self) -> usize {
        self.id
    }
}

impl Id for FilterEntry {
    fn id(&self) -> usize {
        self.text.id
    }
}

impl From<TextEntry> for FilterEntry {
    fn from(value: TextEntry) -> Self {
        Self {
            text_lower: Box::from(value.text().to_lowercase()),
            text: value,
        }
    }
}
