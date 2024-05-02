use recordkeeper_macros::SaveBin;

use crate::{character::CHARACTER_MAX, util::FixStr, SAVE_SLOT_COUNT};

const ORIGIN_SHARD_MAX: usize = 7;

#[derive(SaveBin, Debug)]
pub struct PlayReportStats {
    pub data_sequence_key: [u8; 8],
    pub repeat_play_count: u32,
    pub product_mode: FixStr<16>,
    /// Monotonic Silver Nopon Coin count
    pub silver_coin_total: u64,
    /// Monotonic Gold Nopon Coin count
    pub gold_coin_total: u64,
    /// Monotonic Ether Cylinder count
    pub ether_cylinder_total: u64,
    /// Monotonic count of each Origin Shard type (16083-16089)
    pub origin_shard_total: [u64; ORIGIN_SHARD_MAX],

    pub camp_levelup_total: u64,
    pub cooking_total: u64,
    pub gem_crafting_total: u64,
    pub gem_levelup_total: u64,
    pub gold_total: u64,
    /// Times a collectopedia request was fulfilled
    pub collepedia_total: u64,
    /// Times a collectopedia first-time reward was obtained
    pub collepedia_first_total: u64,

    /// Times this playthrough was saved in each slot
    pub save_count: [u64; SAVE_SLOT_COUNT],

    pub sp_total: u64,
    /// Total number of battles (from initial enemy to full deaggro)
    pub battles_total: u64,
    /// Total victories against regular enemies
    pub defeated_normal_total: u64,
    /// Total victories against unique monsters
    pub defeated_unique_total: u64,
    /// Total victories against bosses
    pub defeated_boss_total: u64,
    pub class_levelup_total: u64,

    pub settings_change_total: u64,
    /// Times a new location was discovered.
    pub location_total: u64,
    /// Times a collectible item was picked up.
    ///
    /// This is always incremented by 1, so it is not affected by Collectible Boost.
    pub collection_total: u64,
    /// Monotonic DX Ether Cylinder (DLC2) count
    pub dx_ether_cylinder_total: u64,
    pub ether_sphere_total: u64,
    /// Monotonic crafted accessory count (DLC3)
    pub accessory_craft_total: u64,
    // Not sure: times crafted accessories were upgraded
    accessory_craft_enhance_total: u64,
    // Not sure: times crafted accessories were dismantled
    accessory_craft_dismantle_total: u64,
    /// Monotonic DLC4 Affinity Point count
    pub dlc4_ap_total: u64,

    #[loc(0x220)]
    pub character_growth: [CharacterGrowth; CHARACTER_MAX],
}

#[derive(SaveBin, Debug)]
pub struct CharacterGrowth {
    /// Total experience gained by the character
    pub exp_total: u64,
    /// Total class experience gained by the character (usually 0 for heroes)
    pub class_exp_total: u64,
}
