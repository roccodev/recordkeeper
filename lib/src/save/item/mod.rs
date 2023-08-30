use recordkeeper_macros::SaveBin;
use thiserror::Error;

use crate::dlc::CRAFTED_ITEM_ID;

pub const ITEM_ACCESSORY_MAX: usize = 1500;

pub mod edit;

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
    pub accessories: [ItemSlot; ITEM_ACCESSORY_MAX],
    /// `ITM_Precious`
    pub key_items: [ItemSlot; 200],
    /// `ITM_Exchange` (unused item type)
    pub exchange: [ItemSlot; 16],
    /// `ITM_Extra`
    pub extra: [ItemSlot; 64],
}

/// An item slot in the player's inventory.
///
/// To edit item slots, use the [`edit::ItemEditor`] struct.
#[derive(SaveBin, Debug)]
#[size(16)]
pub struct ItemSlot {
    item_id: u16,
    slot_index: u16,
    item_type: u32,
    chronological_id: u32,
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

#[derive(SaveBin, Debug)]
pub struct DlcManualSlot {
    item_id: u16,
    inventory_slot_index: u16,
    item_type: u16,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ItemType {
    Cylinder = 1,
    Gem = 2,
    Collection = 3,
    Info = 4,
    Accessory = 5,
    Collectopedia = 6,
    Precious = 7,
    Exchange = 8,
    Extra = 9,
}

#[derive(Error, Debug)]
#[error("unknown item type {0}")]
pub struct TypeFromIntError(u32);

impl Inventory {
    pub fn slots(&self, item_type: ItemType) -> &[ItemSlot] {
        match item_type {
            ItemType::Cylinder => &self.cylinders,
            ItemType::Gem => &self.gems,
            ItemType::Collection => &self.collectibles,
            ItemType::Info => &self.infos,
            ItemType::Accessory => &self.accessories,
            ItemType::Precious => &self.key_items,
            ItemType::Exchange => &self.exchange,
            ItemType::Extra => &self.extra,
            t => panic!("unsupported item type {t:?}"),
        }
    }

    pub fn slots_mut(&mut self, item_type: ItemType) -> &mut [ItemSlot] {
        match item_type {
            ItemType::Cylinder => &mut self.cylinders,
            ItemType::Gem => &mut self.gems,
            ItemType::Collection => &mut self.collectibles,
            ItemType::Info => &mut self.infos,
            ItemType::Accessory => &mut self.accessories,
            ItemType::Precious => &mut self.key_items,
            ItemType::Exchange => &mut self.exchange,
            ItemType::Extra => &mut self.extra,
            t => panic!("unsupported item type {t:?}"),
        }
    }
}

impl ItemSlot {
    /// Returns the slot's item ID.
    pub fn item_id(&self) -> u16 {
        self.item_id
    }

    /// Returns the slot's item amount.
    pub fn amount(&self) -> u16 {
        self.amount
    }

    /// Returns the slot's item type.
    ///
    /// ## Panics
    /// Panics if the item type is invalid or if the slot is empty.
    pub fn item_type(&self) -> ItemType {
        assert!(self.is_valid(), "empty slot");
        ItemType::try_from(self.item_type).unwrap()
    }

    /// Returns whether the slot is occupied by a valid item.
    pub fn is_valid(&self) -> bool {
        self.flags & (SlotFlags::Active as u8) != 0
    }

    /// Returns whether the slot hosts a crafted accessory. (DLC3)
    pub fn is_crafted_accessory(&self) -> bool {
        self.is_valid()
            && self.item_type() == ItemType::Accessory
            && self.item_id() == CRAFTED_ITEM_ID
    }
}

impl ItemType {
    pub fn lang_id(self) -> &'static str {
        match self {
            Self::Cylinder => "cylinder",
            Self::Gem => "gem",
            Self::Collection => "collection",
            Self::Collectopedia => "collepedia",
            Self::Info => "info",
            Self::Accessory => "accessory",
            Self::Precious => "precious",
            Self::Exchange => "exchange",
            Self::Extra => "extra",
        }
    }
}

impl TryFrom<u32> for ItemType {
    type Error = TypeFromIntError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::Cylinder,
            2 => Self::Gem,
            3 => Self::Collection,
            4 => Self::Info,
            5 => Self::Accessory,
            6 => Self::Precious,
            7 => Self::Exchange,
            9 => Self::Extra,
            9 => Self::Collectopedia,
            i => return Err(TypeFromIntError(i)),
        })
    }
}
