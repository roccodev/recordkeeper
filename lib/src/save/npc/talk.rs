use recordkeeper_macros::SaveBin;

use crate::flags::BitFlags;

#[derive(SaveBin, Debug)]
pub struct NpcTalkFlags {
    /// One flag per entry in `FLD_NpcList`
    npcs_interacted_with: BitFlags<1, 32>,
    /// 11k flags (`FLD_NpcTalkResource`, 11 per row, BaseEvent + 10 extra events)
    talk_resources_read: BitFlags<1, 344>,
}
