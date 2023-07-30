use recordkeeper_macros::SaveBin;

pub const CHARACTER_MAX: usize = 64;
const CHARACTER_CLASS_MAX: usize = 64;

#[derive(SaveBin)]
pub struct Character {
    level: u32,
    exp: u32,
    bonus_exp: u32,

    #[loc(0x10)]
    selected_class: u8,

    #[loc(0x14)]
    class_inventory: [CharacterClass; CHARACTER_CLASS_MAX],
}

#[derive(SaveBin)]
pub struct CharacterClass {
    cp: u32,
    attachments: u16, // unsure
    level: u8,

    #[loc(0x8)]
    gems: [u8; 3],
    arts: [u16; 8],
    skills: [u16; 7]
}