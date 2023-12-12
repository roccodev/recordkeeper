use recordkeeper_macros::SaveBin;

use crate::util::FixVec;

pub mod amiibo;

#[derive(SaveBin, Debug)]
pub struct ChronologicalData {
    /// Class unlock order for each main character
    /// (0 is Noah for base game saves, Matthew for DLC4 saves.)
    pub character_classes: ChronologicalTable<64, 6>,

    #[loc(0x302)]
    art_status_max: u16,
    skill_status_max: u16,

    /// Main character art unlock order, uses `art_status_max`
    character_art_status: TableInner<1000, 6>,
    /// Main character skill unlock order, uses `skill_status_max`
    #[loc(0x31e6)]
    character_skill_status: TableInner<500, 6>,

    /// Art unlock order for each hero. The vectors must retain their
    /// sorted order. Uses `art_status_max` as the current maximum.
    #[loc(0x4958)]
    hero_art_status: [FixVec<IdSortPair, 64>; 58],
    /// Skill unlock order for each hero. The vectors must retain their
    /// sorted order. Uses `skill_status_max` as the current maximum.
    #[loc(0x8528)]
    hero_skill_status: [FixVec<IdSortPair, 64>; 58],

    /// Soulhacker art unlock order, uses `art_status_max`
    #[loc(0xc0f8)]
    soul_hack_art_status: TableInner<100, 1>,
    /// Soulhacker skill unlock order, uses `skill_status_max`
    #[loc(0xc1c0)]
    soul_hack_skill_status: TableInner<100, 1>,

    /// Character unlock order
    #[loc(0xc288)]
    pub unlocked_characters: ChronologicalTable<64>,

    /// NPC collectopedia entry unlock order (ID from `FLD_KizunaNpc` - 1)
    #[loc(0xc30a)]
    pub npc_collectopedia: ChronologicalTable<500>,

    /// Quest unlock order
    #[loc(0xc6f4)]
    pub quests: ChronologicalTable<600>,
}

#[derive(SaveBin, Debug)]
pub struct ChronologicalTable<const R: usize, const C: usize = 1> {
    max: u16,
    items: TableInner<R, C>,
}

#[derive(SaveBin, Debug)]
pub struct IdSortPair {
    pub id: u16,
    pub sort: u16,
}

#[derive(SaveBin, Debug)]
struct TableInner<const R: usize, const C: usize> {
    buf: [[u16; R]; C],
}
