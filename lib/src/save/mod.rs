use crate::character::{PARTY_GUEST_MAX, PARTY_MAX};
use crate::error::SaveError;
use crate::flags::UnknownFlags;
use crate::item::Inventory;
use crate::save::character::{Character, Ouroboros, CHARACTER_MAX, OUROBOROS_MAX};
use crate::save::enemy::{EnemyTombstone, ENEMY_TOMBSTONE_MAX};
use crate::save::flags::AllFlags;

use dlc::Dlc4;
use recordkeeper_macros::SaveBin;

pub mod character;
pub mod dlc;
pub mod enemy;
pub mod flags;
pub mod item;

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
    play_time: PlayTime,
    #[loc(0x18)]
    timestamp: SaveTimestamp,
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
    #[loc(0x6a8)]
    pub ship_pos: Pos,

    #[loc(0x710)]
    pub flags: AllFlags,

    #[loc(0xe330)]
    pub party_character_ids: [u16; PARTY_MAX],
    pub party_character_count: u64,

    /// Guest IDs from FLD_NpcList
    #[loc(0xe358)]
    pub party_guest_ids: [u16; PARTY_GUEST_MAX],
    pub party_guest_count: u64,

    #[loc(0xe3a0)]
    pub characters: [Character; CHARACTER_MAX],
    pub ouroboros: [Ouroboros; OUROBOROS_MAX],

    #[loc(0x53c78)]
    pub inventory: Inventory,

    #[loc(0x181c80)]
    pub unknown_flags: UnknownFlags,

    #[loc(0x183000)]
    pub enemy_tombstones: [EnemyTombstone; ENEMY_TOMBSTONE_MAX],

    #[loc(0x1bec5c)]
    pub dlc4: Dlc4,
}

#[derive(SaveBin, Debug)]
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
