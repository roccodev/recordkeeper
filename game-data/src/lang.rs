use std::rc::Rc;

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

/// A key to sort entries based on their translated name
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SortKey {
    ids: Rc<[usize]>,
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

impl<T> Table<T> {
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

impl SortKey {
    /// Lists the items in the order given by the language sort key.
    pub fn list<'i, T: Id>(&self, items: &'i [T]) -> Vec<&'i T> {
        let mut items: Vec<_> = items.into_iter().collect();
        items.sort_unstable_by_key(|i| i.id());
        let mut sorted = Vec::with_capacity(items.len());
        for id in self.ids.iter() {
            let idx = items
                .binary_search_by_key(id, |i| i.id())
                .expect("id not found in items");
            sorted.push(items[idx]);
        }
        sorted
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

impl<'a> From<&'a FilterEntry> for &'a TextEntry {
    fn from(value: &FilterEntry) -> &TextEntry {
        &value.text
    }
}

impl FromIterator<usize> for SortKey {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        Self {
            ids: iter.into_iter().collect(),
        }
    }
}
