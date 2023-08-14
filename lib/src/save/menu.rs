use crate::flags::BitFlags;
use crate::time::AmiiboTimeData;
use recordkeeper_macros::SaveBin;

const FLAG_1_BIT_COUNT: usize = 20000;
const FLAG_2_BIT_COUNT: usize = 4512;
const QUICK_ACTIONS_MAX: usize = 4;

#[derive(SaveBin, Debug)]
pub struct MenuData {
    /// Menu flags, mostly for the little dot that signals new content in a page.
    pub flags: MenuFlags,
    /// Keybinds for quick actions.
    ///
    /// Order is XYBA. Values are row indices for the quick action BDAT tables.
    ///
    /// ## See also
    /// Table `7E6F5DCC` (base game), `B1F2B1E7` (Future Redeemed)
    #[loc(0xe2c)]
    pub quick_actions: [u8; QUICK_ACTIONS_MAX],
    #[loc(0xe30)]
    pub amiibo_time_data: AmiiboTimeData,
}

#[derive(SaveBin, Debug)]
pub struct MenuFlags {
    flags_1b: BitFlags<1, { (FLAG_1_BIT_COUNT + 31) / 32 }>,
    flags_2b: BitFlags<2, { (FLAG_2_BIT_COUNT + 15) / 16 }>,
}
