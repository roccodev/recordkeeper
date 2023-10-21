use crate::{lang::text_table_from_bdat, BdatRegistry, LangBdatRegistry, ModernRow};
use bdat::label_hash;
use game_data::enhance::{Enhance, EnhanceEffect, EnhanceLang, EnhanceRegistry};

pub fn load_enhance(bdat: &BdatRegistry) -> EnhanceRegistry {
    let mut registry = EnhanceRegistry::default();

    let table = bdat.table(label_hash!("BTL_Enhance"));
    table
        .rows()
        .map(|row| read_enhance(row))
        .for_each(|item| registry.register_instance(item));

    let table = bdat.table(label_hash!("BTL_EnhanceEff"));
    table
        .rows()
        .map(|row| read_enhance_effect(row))
        .for_each(|item| registry.register_effect(item));

    registry
}

pub fn load_enhance_lang(bdat: &LangBdatRegistry) -> EnhanceLang {
    let captions = bdat.table(label_hash!("msg_btl_enhance_cap"));
    EnhanceLang::new(text_table_from_bdat(captions))
}

fn read_enhance(row: ModernRow) -> Enhance {
    let effect_id = row.get(label_hash!("EnhanceEffect")).to_integer();
    let caption_id = row.get(label_hash!("Caption")).to_integer();

    let param_1 = row.get(label_hash!("Param1")).to_float();
    let param_2 = row.get(label_hash!("Param2")).to_float();

    Enhance {
        instance_id: row.id() as u32,
        effect_id,
        caption_id,
        param_1,
        param_2,
    }
}

fn read_enhance_effect(row: ModernRow) -> EnhanceEffect {
    let param = row.get(label_hash!("Param")).to_integer();
    EnhanceEffect {
        id: row.id() as u32,
        param: param as u16,
    }
}
