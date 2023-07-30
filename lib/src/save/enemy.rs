use recordkeeper_macros::SaveBin;

pub const ENEMY_TOMBSTONE_MAX: usize = 200;

#[derive(SaveBin)]
pub struct EnemyTombstone {
    /// Highest level rematches, 4 bits for each difficulty
    rematches: [u8; 2],
    defeated: bool,
    best_time: u16,
    best_time_highest_level: u16
}