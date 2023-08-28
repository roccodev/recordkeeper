use recordkeeper_macros::SaveBin;

#[derive(SaveBin, Debug)]
pub struct Inventory {
    #[loc(0x28)]
    /// `ITM_Cylinder`
    pub cylinders: [ItemSlot; 16],
    /// `ITM_Gem`
    pub gems: [ItemSlot; 300],
    /// `ITM_Collection`
    pub collectibles: [ItemSlot; 1500],
    /// `ITM_Info`, discussion info dialogues
    pub infos: [ItemSlot; 800],
    /// `ITM_Accessory`
    pub accessories: [ItemSlot; 1500],
    /// `ITM_Precious`
    pub key_items: [ItemSlot; 200],
    /// `ITM_Exchange` (unused item type)
    pub exchange: [ItemSlot; 16],
    /// `ITM_Extra`
    pub extra: [ItemSlot; 64],
}

#[derive(SaveBin, Debug)]
#[size(16)]
pub struct ItemSlot {
    pub item_id: u16,
    pub slot_index: u16,
    item_type: u32,
    sort_key: u32, // unsure
    #[loc(0xc)]
    pub amount: u16,
    flags: u8,
}

enum SlotFlags {
    /// The slot has an item inside
    Active = 1,
    /// The player has marked the item as favorite
    Favorite = 1 << 2,
    /// The small circle icon for "unchecked" items
    New = 1 << 3,
}

#[derive(SaveBin, Debug)]
pub struct DlcManualSlot {
    item_id: u16,
    inventory_slot_index: u16,
    item_type: u16,
}

impl ItemSlot {
    pub fn is_valid(&self) -> bool {
        self.flags & (SlotFlags::Active as u8) != 0
    }

    pub fn edit(&mut self, slot_id: u16, item_type: u32, editor: impl Fn(&mut ItemSlot)) {
        editor(self);
        if self.item_id == 0 || self.amount == 0 {
            self.clear();
        } else {
            self.slot_index = slot_id;
            self.item_type = item_type;
            self.flags |= SlotFlags::Active as u8;
        }
    }

    pub fn clear(&mut self) {
        self.item_id = 0;
        self.amount = 0;
        self.sort_key = 0;
        self.item_type = 0;
        self.flags &= !(SlotFlags::Active as u8);
    }
}
