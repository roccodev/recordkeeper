use recordkeeper_macros::SaveBin;

pub const CHARACTER_MAX: usize = 64;
const CHARACTER_CLASS_MAX: usize = 64;

#[derive(SaveBin)]
#[size(4444)]
pub struct Character {
    pub level: u32,
    pub exp: u32,
    pub bonus_exp: u32,

    #[loc(0x10)]
    pub selected_class: u8,

    #[loc(0x14)]
    pub class_inventory: [CharacterClass; CHARACTER_CLASS_MAX],

    pub costume_id: u16,
    /// The level the character joined the party at. Seems to be the ending party's level for NG+
    pub arrival_level: u8,
    pub dirty_level: u8,
    pub attachment: u8, // unsure
}

#[derive(SaveBin)]
#[size(68)]
pub struct CharacterClass {
    cp: u32,
    attachments: u16, // unsure
    level: u8,

    #[loc(0x8)]
    gems: [u8; 10], // ? (-1 for locked slot probably)
    skills: [u16; 7],
    arts: [u16; 8],

    accessories: [ClassAccessory; 3],
}

#[derive(SaveBin)]
pub struct ClassAccessory {
    bdat_id: u16,
    slot_index: u16,
    item_type: u16,
}
