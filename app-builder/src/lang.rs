use std::cmp::Reverse;

use bdat::{label_hash, modern::ModernTable};
use game_data::{
    lang::{FilterEntry, FilterTable, SortKey, TextEntry, TextTable},
    IdInt,
};

pub fn text_table_from_bdat(table: &ModernTable) -> TextTable {
    TextTable::new(table.rows().filter_map(|row| {
        let text = row.get(label_hash!("name")).as_str();
        (!text.is_empty()).then(|| TextEntry::new(text, row.id()))
    }))
}

pub fn filter_table_from_bdat(table: &ModernTable) -> FilterTable {
    FilterTable::new(table.rows().filter_map(|row| {
        let text = row.get(label_hash!("name")).as_str();
        (!text.is_empty()).then(|| FilterEntry::new(text, row.id()))
    }))
}

/// Generates a sort key based on (ID, name ID) pairs. Items with no name
/// are ordered last.
pub fn sort_key_from_bdat(
    lang_table: &ModernTable,
    id_name_pairs: impl IntoIterator<Item = (IdInt, IdInt)>,
) -> SortKey {
    let mut id_strs: Vec<_> = id_name_pairs
        .into_iter()
        .map(|(id, name_id)| {
            let name = lang_table.get_row(name_id).and_then(|row| {
                let name = row.get(label_hash!("name")).as_str();
                (!name.is_empty()).then_some(name)
            });
            (id, name)
        })
        .collect();
    // TODO: collator to handle JP alphabetical ordering
    // Options are None-first, so we first sort strings in reverse alphabetical order,
    // then reverse the whole order.
    id_strs.sort_unstable_by_key(|(_, name)| Reverse(name.map(Reverse)));
    id_strs.into_iter().map(|(id, _)| id).collect()
}
