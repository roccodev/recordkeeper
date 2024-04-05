use serde::{Deserialize, Serialize};

use crate::IdInt;

#[derive(Serialize, Deserialize)]
pub struct ManualData {
    pub flags: Flags,
}

#[derive(Serialize, Deserialize)]
pub struct Flags {
    pub new_game_plus: Flag,
    pub difficulty: Flag,
    pub scenario: Flag,
    /// Enables Interlink for Noah
    pub ouro_enable_noah: Flag,
    /// Base flag to enable Interlink for each character.
    ///
    /// Note: the base flag refers to Mio
    pub ouro_enable: Flag,
    /// Location unlocked base flag.
    /// Flag ID = Base + (ma*a_GMK_Location row ID - 1)
    pub location: Flag,
    pub game_clear: Flag,
    pub landmark_count: Flag,
    pub secret_count: Flag,
    /// Main game flag, set when a save is loaded/created and there is
    /// a completed Future Redeemed save.
    pub fr_complete: Flag,
    pub dx_cylinder_count: Flag,
    pub dx_cylinder_level: Flag,
    pub affinity_points: Flag,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Flag {
    pub bits: usize,
    pub index: IdInt,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct FlagRange {
    pub flag: Flag,
    pub count: usize,
}
