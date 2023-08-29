use recordkeeper_macros::SaveBin;

use super::{EmblemItem, Gauntlet, GauntletState, EMBLEM_MAX};

const CHALLENGE_BATTLE_NUM_CHALLENGES: usize = 18;
pub const CHALLENGE_BATTLE_NUM_GAUNTLET: usize = 4;
pub const CHALLENGE_BATTLE_NUM_GAUNTLET_STATES: usize = 1; // likely a 1-item array in the game
pub const CHALLENGE_BATTLE_DIFFICULTY_MAX: usize = 3;

#[derive(SaveBin, Debug)]
#[size(28900)]
pub struct ChallengeBattle {
    #[assert(6)]
    _unk: u32,
    challenges_1_18: [Challenge; CHALLENGE_BATTLE_NUM_CHALLENGES],

    gauntlet: [Gauntlet; CHALLENGE_BATTLE_NUM_GAUNTLET],

    #[loc(0x368)]
    gauntlet_states: [GauntletState; CHALLENGE_BATTLE_NUM_GAUNTLET_STATES],

    #[loc(0x6320)]
    challenges_19: [Challenge; 1], // easier to work with

    /// Actual size: number of rows in `BTL_ChSU_Emblem`
    #[loc(0x658c)]
    emblem_shop: [EmblemItem; EMBLEM_MAX],

    #[loc(0x70cc)]
    nopon_stone_red: u32,
    nopon_stone_blue: u32,
    // Two flags here for whether the current challenge has a bonus, not relevant for saves
}

#[derive(SaveBin, Debug)]
#[size(32)]
pub struct Challenge {
    ranks: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX], // TODO enum
    best_time: [f32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    clear_count: u32,
    flags: [bool; 4], // #3: whether the challenge currently has a bonus
}
