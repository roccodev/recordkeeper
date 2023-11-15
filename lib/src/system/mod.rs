use recordkeeper_macros::SaveBin;

use crate::{error::SaveError, flags::BitFlags};

pub(crate) const SYSTEM_VERSION: u32 = 2;

#[derive(SaveBin, Debug)]
pub struct SystemData {
    #[assert(0xe6ab6074)]
    _magic: u32,
    #[assert(SYSTEM_VERSION, SaveError::UnsupportedVersion(ACTUAL, SYSTEM_VERSION))]
    version: u32,

    flags: BitFlags<1, 2>,

    /// Index is `option_id` from `MNU_option_*`
    #[loc(0x10)]
    global_settings: [u16; 80],

    #[loc(0xb8)]
    some_flags: BitFlags<1, 30>,

    #[loc(0x138)]
    event_theater_flags: BitFlags<2, 313>,

    /// ID for `MNU_PatchInfo`
    #[loc(0x61c)]
    latest_patch_id: u16,

    /// Index is ID in `RSC_PcCostumeOpen`. In each byte, each bit is for each character.
    #[loc(0x66c)]
    costume_characters: [u8; 52],

    #[loc(0x6a0)]
    _unk: u64, // new game count?
    /// One for each slot
    save_count: [u64; 5],
    /// One for each slot
    load_count: [u64; 5],
    pub continue_count: u64,
    pub settings_save_count: u64,
}

pub enum SystemFlag {
    /// Whether the main game has been cleared on any file.
    MainGameClear = 0,
    /// Whether the main game has been cleared on any NG+ file.
    MainGameClearNgp = 1,
    EnableCostumes = 2,
    /// Whether rewards for the Shulk amiibo have been claimed on any file.
    AmiiboRewards1 = 3,
    /// Whether rewards for the Pyra amiibo have been claimed on any file.
    AmiiboRewards2 = 4,
    /// Whether rewards for the Mythra amiibo have been claimed on any file.
    AmiiboRewards3 = 5,
    /// Whether Future Redeemed has been cleared on any file.
    Dlc4Clear = 6,
    /// Whether Future Redeemed has been cleared on any NG+ file.
    Dlc4ClearNgp = 7,
    /// Whether a row with ID `E7BA87FE` exists in `RSC_PcCostumeOpen`
    Amiibo4Support = 8,
}

impl SystemData {
    pub fn is_flag_set(&self, flag: SystemFlag) -> bool {
        self.flags.get(flag as usize).unwrap() != 0
    }

    pub fn set_flag(&mut self, flag: SystemFlag, value: bool) {
        self.flags.set(flag as usize, u8::from(value).into())
    }
}
