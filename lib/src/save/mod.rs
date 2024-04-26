use crate::error::SaveError;
use crate::item::Inventory;
use crate::save::character::{Character, Ouroboros, CHARACTER_MAX, OUROBOROS_MAX};
use crate::save::enemy::{EnemyTombstone, ENEMY_TOMBSTONE_MAX};
use crate::save::flags::AllFlags;
use crate::{
    character::{
        formation::{PartyFormation, PARTY_FORMATION_MAX},
        PARTY_GUEST_MAX, PARTY_MAX,
    },
    io::SaveBin,
};

use crate::menu::MenuData;
use crate::util::FixVec;
use recordkeeper_macros::SaveBin;

use self::character::CharacterSets;
use self::chrono::ChronologicalData;
use self::enemy::SoulHackAchievements;
use self::field::FieldConfig;
use self::flags::BitFlags;
use self::item::GemLevels;
use dlc::{AccessoryCrafting, ChallengeBattle, Dlc4, PowAugment, POW_AUGMENT_NUM};

pub mod character;
pub mod chrono;
pub mod dlc;
pub mod enemy;
pub mod field;
pub mod flags;
pub mod item;
pub mod menu;

pub(crate) const SAVE_VERSION: u8 = 10;
pub(crate) const SAVE_MAGIC: [u8; 4] = [0x6a, 0xfa, 0x68, 0xb3];

/// Defines the save file binary structure.
///
/// This struct should not be created manually, as it's quite big and requires substantial stack
/// space. Instead, it is recommended to use [`SaveFile::from_bytes`] to get a heap-allocated
/// save file.
///
/// [`SaveFile::from_bytes`]: crate::SaveFile::from_bytes
#[derive(SaveBin, Debug)]
pub struct SaveData {
    #[assert(SAVE_MAGIC)]
    _magic: [u8; 4],
    #[assert(SAVE_VERSION, SaveError::UnsupportedVersion(ACTUAL as u32, SAVE_VERSION as u32))]
    save_version: u8,

    #[loc(0x10)]
    pub play_time: PlayTime,
    #[loc(0x18)]
    pub timestamp: SaveTimestamp,
    pub gold: u32,
    /// 0-100: at 100 a new cylinder is generated when harvesting ether.
    pub ether_cylinder_progress: u16,

    /// ID for `SYS_MapJumpList`
    pub respawn_point: u16,

    /// Updated by the game on load (only adding new entries, removing none).  
    /// Also recomputed from colony IconFlags when creating a NG+ file.
    #[loc(0x4c)]
    pub liberated_colonies: BitFlags<1, 1>,

    /// For Ino's DX cylinders. (DLC2)
    ///
    /// 0-100: at 100 a new cylinder is generated when harvesting ether.
    pub ether_cylinder_dx_progress: u16,

    /// Max chronological ID for Future Redeemed NPC community entries.
    ///
    /// The next community entry is given this chronological ID, which is then
    /// incremented by one.
    #[loc(0x54)]
    pub(crate) dlc4_community_order_max: u16,

    #[loc(0x664)]
    save_flags: BitFlags<1, 1>,

    /// Saved event flow ID for end-of-chapter saves
    #[loc(0x684)]
    pub saved_event_flow: u32,

    #[loc(0x68c)]
    pub map_id: u16,
    pub map_time: MapTime,

    /// ID for `RSC_WeatherSet`. The game only considers this if
    /// [`SaveFlag::WeatherLocked`] is set.
    pub weather: u16,

    #[loc(0x6a0)]
    pub player_pos: Pos,
    #[loc(0x6c0)]
    pub ship_pos: Pos,

    /// Starts at 0, index for `party_characters`
    pub controlled_character_idx: u32,

    #[loc(0x710)]
    pub flags: AllFlags,

    /// Character IDs in the active party
    #[loc(0xe330)]
    pub party_characters: FixVec<u16, PARTY_MAX>,
    /// Guest IDs from `FLD_NpcList`
    #[loc(0xe358)]
    pub party_guests: FixVec<u16, PARTY_GUEST_MAX>,

    #[loc(0xe370)]
    pub character_sets: CharacterSets,

    #[loc(0xe3a0)]
    pub characters: Box<[Character; CHARACTER_MAX]>,
    pub ouroboros: Box<[Ouroboros; OUROBOROS_MAX]>,
    pub gem_levels: GemLevels,

    /// ID for `BTL_Pair`
    #[loc(0x53c38)]
    dlc4_pairs: [u8; 3],

    #[loc(0x53c78)]
    pub inventory: Inventory,

    #[loc(0x7c760)]
    pub soul_hack_achievements: SoulHackAchievements,

    #[loc(0x7d000)]
    pub field_config: FieldConfig,

    #[cfg(feature = "map-bitmaps")]
    #[loc(0x7e000)]
    pub map_visibility: field::map::MapBitmaps,

    #[loc(0x181c80)]
    pub menu_data: MenuData,

    #[loc(0x183000)]
    pub enemy_tombstones: [EnemyTombstone; ENEMY_TOMBSTONE_MAX],

    #[loc(0x184648)]
    pub chronological_data: ChronologicalData,

    #[loc(0x1911f0)]
    pub pow_augment: [PowAugment; POW_AUGMENT_NUM],

    #[loc(0x191250)]
    pub accessory_crafting: AccessoryCrafting,

    #[loc(0x193ed8)]
    pub challenge_battle: ChallengeBattle,

    #[loc(0x19afc0)]
    pub party_formations: Box<[PartyFormation; PARTY_FORMATION_MAX]>,

    #[loc(0x1bec5c)]
    pub dlc4: Dlc4,
}

#[derive(SaveBin, Debug, Clone, Copy)]
pub struct PlayTime {
    raw: u32,
}

#[derive(SaveBin, Debug, Clone, Copy, PartialEq)]
pub struct SaveTimestamp {
    time: u32,
    date: u32,
}

#[derive(SaveBin, Debug)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rotation: f32,
}

#[derive(SaveBin, Debug)]
pub struct MapTime<N = u16>
where
    N: SaveBin,
    SaveError: From<<N as SaveBin>::ReadError>,
    SaveError: From<<N as SaveBin>::WriteError>,
{
    pub hour: N,
    pub minute: N,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SaveFlag {
    TimeLocked = 0,
    WeatherLocked = 1,
    AboardShip = 2,
    /// Saved when prompted to by the game, for example
    /// on chapter end or before the final fight.
    Intermission = 3,
    Dlc4 = 4,
    /// Mid-run data from the gauntlet exists.
    Gauntlet = 5,
}

impl SaveData {
    pub fn is_flag_set(&self, flag: SaveFlag) -> bool {
        self.save_flags
            .get(flag as usize)
            .map(|v| v != 0)
            .expect("invalid flag index")
    }

    pub fn set_flag(&mut self, flag: SaveFlag, value: bool) {
        self.save_flags.set(flag as usize, u8::from(value).into())
    }

    /// Returns whether the save file is a Future Redeemed file.
    pub fn is_dlc4(&self) -> bool {
        self.is_flag_set(SaveFlag::Dlc4)
    }

    /// Returns whether there currently is a mid-run gauntlet save.
    pub fn has_gauntlet_save(&self) -> bool {
        self.is_flag_set(SaveFlag::Gauntlet) && self.challenge_battle.gauntlet_save().active
    }
}

impl PlayTime {
    pub fn from_seconds(seconds: u32) -> Self {
        Self { raw: seconds }
    }

    pub fn to_seconds(self) -> u32 {
        self.raw
    }

    pub fn to_hours_mins_secs(self) -> (u32, u32, u32) {
        let secs = self.to_seconds();
        (secs / 3600, secs % 3600 / 60, secs % 3600 % 60)
    }
}

impl SaveTimestamp {
    pub fn from_date_time(year: u32, month: u8, day: u8, hour: u8, minute: u8) -> Self {
        let date = (year << 0x12) | (((month as u32) & 0xf) << 0xe) | (day as u32 & 0x1f);
        let time = ((hour as u32) << 0x1a) | ((minute as u32 & 0x3f) << 0x14);
        Self { date, time }
    }

    pub fn year(&self) -> u32 {
        self.date >> 0x12
    }

    pub fn month(&self) -> u8 {
        (self.date >> 0xe & 0xf) as u8
    }

    pub fn day(&self) -> u8 {
        (self.date & 0x1f) as u8
    }

    pub fn hour(&self) -> u8 {
        (self.time >> 0x1a) as u8
    }

    pub fn minute(&self) -> u8 {
        (self.time >> 0x14 & 0x3f) as u8
    }

    pub fn to_iso_date(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year(), self.month(), self.day())
    }

    pub fn to_iso_time(&self) -> String {
        format!("{:02}:{:02}", self.hour(), self.minute())
    }
}

#[cfg(test)]
mod tests {
    use crate::SaveTimestamp;

    #[test]
    fn test_timestamp() {
        let ts = SaveTimestamp::from_date_time(2023, 1, 2, 12, 28);
        assert_eq!(2023, ts.year());
        assert_eq!(1, ts.month());
        assert_eq!(2, ts.day());
        assert_eq!(12, ts.hour());
        assert_eq!(28, ts.minute());
    }
}
