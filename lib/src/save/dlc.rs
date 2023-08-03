use crate::character::ClassAccessory;
use recordkeeper_macros::SaveBin;

pub const DLC4_ENEMYPEDIA_MAX_EACH: usize = 200;
const CHALLENGE_BATTLE_NUM_CHALLENGES: usize = 18;
const CHALLENGE_BATTLE_NUM_GAUNTLET: usize = 4;
const CHALLENGE_BATTLE_DIFFICULTY_MAX: usize = 3;

#[derive(SaveBin, Debug)]
pub struct Dlc4 {
    /// Number of victories for Enemypedia entries 0-199
    enemypedia_0_199: [u8; DLC4_ENEMYPEDIA_MAX_EACH],

    /// Extra inventory, indexed by character ID
    extra_inventory: [DlcExtraInventory; 64],

    /// Number of victories for Enemypedia entries 200-399
    // lol
    #[loc(0x80c8)]
    enemypedia_200_399: [u8; DLC4_ENEMYPEDIA_MAX_EACH],
}

#[derive(SaveBin, Debug)]
#[size(28900)]
pub struct ChallengeBattle {
    #[assert(6)]
    _unk: u32,
    challenges_1_18: [Challenge; CHALLENGE_BATTLE_NUM_CHALLENGES],

    gauntlet: [Gauntlet; CHALLENGE_BATTLE_NUM_GAUNTLET],

    #[loc(0x6320)]
    challenges_19: [Challenge; 1], // easier to work with
}

#[derive(SaveBin, Debug)]
#[size(32)]
pub struct Challenge {
    ranks: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX], // TODO enum
    best_time: [f32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    clear_count: u32,
    flags: [bool; 4], // #3: whether the challenge currently has a bonus
}

#[derive(SaveBin, Debug)]
#[size(72)]
pub struct Gauntlet {
    ranks: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    stage_reached: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    high_score: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    time: [f32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    play_count: [u32; CHALLENGE_BATTLE_DIFFICULTY_MAX],
    in_progress: bool, // unsure
    _unk1: bool,
    _unk2: bool,
    _unk3: bool, // could be whether rewards for ranks A/B have been received
    _unk4: bool,
    #[loc(0x44)]
    _unk5: u32, // clear count?
}

#[derive(SaveBin, Debug)]
#[size(512)]
pub struct DlcExtraInventory {
    /// Likely indexed by class ID
    battle_manual: [ClassAccessory; 64],
}
