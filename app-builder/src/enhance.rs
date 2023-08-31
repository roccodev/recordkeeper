use crate::{const_hash, lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry};
use bdat::RowRef;
use game_data::enhance::{Enhance, EnhanceEffect, EnhanceLang, EnhanceRegistry};

pub fn load_enhance(bdat: &BdatRegistry) -> EnhanceRegistry {
    let mut registry = EnhanceRegistry::default();

    let table = bdat.table(&const_hash!("BTL_Enhance"));
    table
        .rows()
        .map(|row| read_enhance(table.get_row(row.id()).unwrap()))
        .for_each(|item| registry.register_instance(item));

    let table = bdat.table(&const_hash!("BTL_EnhanceEff"));
    table
        .rows()
        .map(|row| read_enhance_effect(table.get_row(row.id()).unwrap()))
        .for_each(|item| registry.register_effect(item));

    registry
}

pub fn load_enhance_lang(bdat: &LangBdatRegistry) -> EnhanceLang {
    let captions = bdat.table(&const_hash!("msg_btl_enhance_cap"));
    EnhanceLang::new(text_table_from_bdat(captions))
}

fn read_enhance(row: RowRef) -> Enhance {
    let effect_id = row[const_hash!("EnhanceEffect")]
        .as_single()
        .unwrap()
        .to_integer();
    let caption_id = row[const_hash!("Caption")]
        .as_single()
        .unwrap()
        .to_integer();

    let param_1 = row[const_hash!("Param1")].as_single().unwrap().to_float();
    let param_2 = row[const_hash!("Param2")].as_single().unwrap().to_float();

    Enhance {
        instance_id: row.id() as u32,
        effect_id,
        caption_id,
        param_1,
        param_2,
    }
}

fn read_enhance_effect(row: RowRef) -> EnhanceEffect {
    let param = row[const_hash!("Param")].as_single().unwrap().to_integer();
    EnhanceEffect {
        id: row.id() as u32,
        param: param as u16,
    }
}
