use crate::character::{
    CharacterParty, PartyFormation, PARTY_FORMATION_MAX, PARTY_GUEST_MAX, PARTY_MAX,
};
use crate::error::SaveError;
use crate::item::Inventory;
use crate::save::character::{Character, Ouroboros, CHARACTER_MAX, OUROBOROS_MAX};
use crate::save::enemy::{EnemyTombstone, ENEMY_TOMBSTONE_MAX};
use crate::save::flags::AllFlags;
use menu::MenuFlags;

use crate::dlc::ChallengeBattle;
use crate::menu::MenuData;
use dlc::Dlc4;
use recordkeeper_macros::SaveBin;

use self::dlc::{PowAugment, POW_AUGMENT_NUM};

pub mod character;
pub mod dlc;
pub mod enemy;
pub mod flags;
pub mod item;
pub mod menu;
pub mod time;

pub(crate) const SAVE_VERSION: u8 = 10;

/// Defines the save file binary structure.
///
/// This struct should not be created manually, as it's quite big and requires substantial stack
/// space. Instead, it is recommended to use [`SaveFile::from_bytes`] to get a heap-allocated
/// save file.
///
/// [`SaveFile::from_bytes`]: crate::SaveFile::from_bytes
#[derive(SaveBin, Debug)]
pub struct SaveData {
    #[assert(0xb368fa6a)]
    _magic: u32,
    #[assert(SAVE_VERSION, SaveError::UnsupportedVersion(ACTUAL))]
    save_version: u8,

    #[loc(0x10)]
    pub play_time: PlayTime,
    #[loc(0x18)]
    pub timestamp: SaveTimestamp,
    pub gold: u32,

    /// Updated by the game on load.
    #[loc(0x4c)]
    pub seen_colonies: u32,

    /// Saved event flow ID for end-of-chapter saves
    #[loc(0x684)]
    saved_event_flow: u32,

    #[loc(0x68c)]
    map_id: u16,
    map_time: MapTime,

    #[loc(0x6a0)]
    pub player_pos: Pos,
    #[loc(0x6c0)]
    pub ship_pos: Pos,

    #[loc(0x710)]
    pub flags: AllFlags,

    #[loc(0xe330)]
    pub party_characters: CharacterParty<PARTY_MAX>,
    /// Guest IDs from FLD_NpcList
    #[loc(0xe358)]
    pub party_guests: CharacterParty<PARTY_GUEST_MAX>,

    #[loc(0xe3a0)]
    pub characters: [Character; CHARACTER_MAX],
    pub ouroboros: [Ouroboros; OUROBOROS_MAX],

    #[loc(0x53c78)]
    pub inventory: Inventory,

    #[loc(0x181c80)]
    pub menu_data: MenuData,

    #[loc(0x183000)]
    pub enemy_tombstones: [EnemyTombstone; ENEMY_TOMBSTONE_MAX],

    #[loc(0x1911f0)]
    pub pow_augment: [PowAugment; POW_AUGMENT_NUM],

    #[loc(0x193ed8)]
    pub challenge_battle: ChallengeBattle,

    #[loc(0x19afc0)]
    pub party_formations: [PartyFormation; PARTY_FORMATION_MAX],

    #[loc(0x1bec5c)]
    pub dlc4: Dlc4,
}

#[derive(SaveBin, Debug, Clone, Copy)]
pub struct PlayTime {
    raw: u32,
}

#[derive(SaveBin, Debug)]
pub struct SaveTimestamp {
    time: u32,
    date: u32,
}

#[derive(SaveBin, Debug)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(SaveBin, Debug)]
pub struct MapTime {
    hour: u16,
    minute: u16,
}

impl PlayTime {
    pub fn to_seconds(self) -> u32 {
        self.raw
    }

    pub fn to_hours_mins_secs(self) -> (u32, u32, u32) {
        let secs = self.to_seconds();
        (secs / 3600, secs % 3600 / 60, secs % 3600 % 60)
    }
}

impl SaveTimestamp {
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
