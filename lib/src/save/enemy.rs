use crate::item::{DlcExtraInventory, DlcManualSlot};
use recordkeeper_macros::SaveBin;

pub const ENEMY_TOMBSTONE_MAX: usize = 200;

#[derive(SaveBin, Debug)]
pub struct EnemyTombstone {
    /// Highest level rematches, 4 bits for each difficulty
    rematches: [u8; 2],
    /// Unknown. Defeated on any playthrough?
    _unk: bool,
    /// Defeated on this playthrough
    defeated: bool,
    /// One record for each difficulty
    time_records: [TombstoneTime; 4],
}

#[derive(SaveBin, Debug)]
pub struct TombstoneTime {
    best_time: u16,
    best_time_highest_level: u16,
}
