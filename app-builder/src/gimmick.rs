use bdat::{label_hash, modern::ModernRowRef, Label};

pub struct GimmickData {
    pub row_id: u32,
    pub type_hash: u32,
    pub gimmick_id: u32,
    pub sequential_id: u32,
    pub external_id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl GimmickData {
    pub fn new(row: ModernRowRef) -> Self {
        Self {
            row_id: row.id() as u32,
            type_hash: row.get(label_hash!("GimmickType")).get_as(),
            gimmick_id: row.get(label_hash!("GimmickID")).get_as(),
            sequential_id: row.get(label_hash!("SequentialID")).get_as::<u16>() as u32,
            external_id: row.get(Label::Hash(0x6C50B44E)).get_as(),
            x: row.get(label_hash!("X")).get_as(),
            y: row.get(label_hash!("Y")).get_as(),
            z: row.get(label_hash!("Z")).get_as(),
        }
    }
}
