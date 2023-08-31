use crate::const_hash;
use bdat::Table;
use game_data::lang::{FilterEntry, FilterTable, TextEntry, TextTable};

pub fn text_table_from_bdat(table: &Table) -> TextTable {
    TextTable::new(table.rows().filter_map(|row| {
        let row = table.get_row(row.id()).unwrap();
        row[const_hash!("name")]
            .as_single()
            .map(|v| v.as_str())
            .and_then(|s| (!s.is_empty()).then_some(s))
            .map(|text| TextEntry::new(text, row.id()))
    }))
}

pub fn filter_table_from_bdat(table: &Table) -> FilterTable {
    FilterTable::new(table.rows().filter_map(|row| {
        let row = table.get_row(row.id()).unwrap();
        row[const_hash!("name")]
            .as_single()
            .map(|v| v.as_str())
            .and_then(|s| (!s.is_empty()).then_some(s))
            .map(|text| FilterEntry::new(text, row.id()))
    }))
}
