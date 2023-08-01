use recordkeeper_macros::SaveBin;

pub const PARTY_MAX: usize = 16;
pub const PARTY_GUEST_MAX: usize = 8;
pub const CHARACTER_MAX: usize = 64;
pub const OUROBOROS_MAX: usize = 6;
const CHARACTER_CLASS_MAX: usize = 64;

#[derive(SaveBin, Debug)]
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

#[derive(SaveBin, Debug)]
pub struct Ouroboros {
    pub art_ids: [u16; 5],
    pub exp: u16, // ???
    pub sp: u32,
    pub linked_skills: [u16; 2],

    #[loc(0x34)]
    pub skill_tree: OuroborosTree,
}

#[derive(SaveBin, Debug)]
#[size(68)]
pub struct CharacterClass {
    cp: u32,
    unlock_points: u16,
    level: u8,

    #[loc(0x8)]
    gems: [u8; 10], // ? (-1 for locked slot probably)
    arts: [u16; 7],
    skills: [u16; 8],

    accessories: [ClassAccessory; 3],
}

#[derive(SaveBin, Debug)]
pub struct ClassAccessory {
    bdat_id: u16,
    slot_index: u16,
    item_type: u16,
}

#[derive(SaveBin, Debug)]
pub struct OuroborosTree {
    raw: u64,
}
