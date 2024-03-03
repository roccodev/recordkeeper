use bdat::{label_hash, ModernTable};
use game_data::lang::{FilterEntry, FilterTable, TextEntry, TextTable};

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
