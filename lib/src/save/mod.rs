use crate::save::character::{Character, Ouroboros, CHARACTER_MAX, OUROBOROS_MAX};
use crate::save::enemy::{EnemyTombstone, ENEMY_TOMBSTONE_MAX};
use crate::save::flags::AllFlags;
use recordkeeper_macros::SaveBin;

mod character;
mod enemy;
mod flags;
mod item;

const SAVE_VERSION: u8 = 10;

#[derive(SaveBin, Debug)]
pub struct SaveData {
    #[assert(0xb368fa6a)]
    _magic: u32,
    #[assert(SAVE_VERSION)]
    save_version: u8,

    #[loc(0x10)]
    play_time: PlayTime,
    #[loc(0x18)]
    timestamp: SaveTimestamp,
    pub gold: u32,

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
    flags: AllFlags,

    #[loc(0xe3a0)]
    pub characters: [Character; CHARACTER_MAX],
    pub ouroboros: [Ouroboros; OUROBOROS_MAX],

    #[loc(0x183000)]
    pub enemy_tombstones: [EnemyTombstone; ENEMY_TOMBSTONE_MAX],
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
