use recordkeeper_macros::SaveBin;
use std::marker::PhantomData;

#[derive(SaveBin, Debug)]
pub struct Inventory {
    #[loc(0x28)]
    /// `ITM_Cylinder`
    cylinders: [ItemSlot; 16],
    /// `ITM_Gem`
    gems: [ItemSlot; 300],
    /// `ITM_Collection`
    collectibles: [ItemSlot; 1500],
    /// `ITM_Info`, discussion info dialogues
    infos: [ItemSlot; 800],
    /// `ITM_Accessory`
    accessories: [ItemSlot; 1500],
    /// `ITM_Precious`
    key_items: [ItemSlot; 200],
    /// `ITM_Exchange` (unused item type)
    exchange: [ItemSlot; 16],
    /// `ITM_Extra`
    extra: [ItemSlot; 64],
}

#[derive(SaveBin, Debug)]
#[size(16)]
pub struct ItemSlot {
    item_id: u16,
    slot_index: u16,
    item_type: u32,
    sort_key: u32, // unsure
    #[loc(0xc)]
    amount: u16,
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
