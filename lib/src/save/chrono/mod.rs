use std::cmp::Ordering;

use recordkeeper_macros::SaveBin;

use crate::{
    item::{Inventory, ItemType},
    util::FixVec,
};

pub mod amiibo;

#[derive(SaveBin, Debug)]
pub struct ChronologicalData {
    /// Class unlock order for each main character
    /// (0 is Noah for base game saves, Matthew for DLC4 saves.)
    pub character_classes: ChronologicalTable<64, 6>,

    #[loc(0x302)]
    art_status_max: u16,
    skill_status_max: u16,

    /// Main character art unlock order, uses `art_status_max`
    character_art_status: TableInner<1000, 6>,
    /// Main character skill unlock order, uses `skill_status_max`
    #[loc(0x31e6)]
    character_skill_status: TableInner<500, 6>,

    /// Art unlock order for each hero. The vectors must retain their
    /// sorted order. Uses `art_status_max` as the current maximum.
    #[loc(0x4958)]
    hero_art_status: [FixVec<IdSortPair, 64>; 58],
    /// Skill unlock order for each hero. The vectors must retain their
    /// sorted order. Uses `skill_status_max` as the current maximum.
    #[loc(0x8528)]
    hero_skill_status: [FixVec<IdSortPair, 64>; 58],

    /// Soulhacker art unlock order, uses `art_status_max`
    #[loc(0xc0f8)]
    soul_hack_art_status: TableInner<100, 1>,
    /// Soulhacker skill unlock order, uses `skill_status_max`
    #[loc(0xc1c0)]
    soul_hack_skill_status: TableInner<100, 1>,

    /// Character unlock order
    #[loc(0xc288)]
    pub unlocked_characters: ChronologicalList<64>,

    /// NPC collectopedia entry unlock order (ID from `FLD_KizunaNpc` - 1)
    #[loc(0xc30a)]
    pub npc_collectopedia: ChronologicalList<500>,

    /// Quest unlock order
    #[loc(0xc6f4)]
    pub quests: ChronologicalList<600>,
}

#[derive(SaveBin, Debug)]
pub struct IdSortPair {
    pub id: u16,
    pub sort: u16,
}

#[derive(SaveBin, Debug)]
pub struct ChronologicalList<const R: usize> {
    max: u16,
    items: TableInner<R, 1>,
}

#[derive(SaveBin, Debug)]
pub struct ChronologicalTable<const R: usize, const C: usize> {
    max: u16,
    items: TableInner<R, C>,
}

/// Defines basic operations on a chronological order table.
pub trait ChronologicalOrder {
    /// Compares two entries based on the order defined by the table, following
    /// descending order, with most recent entries appearing first.
    ///
    /// Entries that are missing from the table appear at the end of the order,
    /// i.e. they are considered greater than every entry.
    ///
    /// ## Panics
    /// This function panics if either entry ID is outside the bounds of this table.
    fn cmp_entries(&self, id_a: usize, id_b: usize) -> Ordering;

    /// Swaps the order of two entries.
    ///
    /// This does nothing if either entry is absent from the table.
    ///
    /// For example, if entry A is #1 on the list, and entry B is #10, swapping
    /// the order of the two entries will make B the new first entry, and move
    /// A to #10.
    ///
    /// ## Panics
    /// This function panics if either entry ID is outside the bounds of this table.
    fn swap(&mut self, id_a: usize, id_b: usize);

    /// Registers a new entry in the table. The new entry appears at the start
    /// of the order.
    ///
    /// ## Panics
    /// This function panics if the entry ID is outside the bounds of this table.
    fn insert(&mut self, id: usize);
}

pub struct CharacterChronoAccessor<'a, const R: usize, const C: usize> {
    max: &'a mut u16,
    table: CharacterChrono<'a, R, C>,
}

enum CharacterChrono<'a, const R: usize, const C: usize> {
    Table(&'a mut TableInner<R, C>),
    Hero(&'a mut [FixVec<IdSortPair, R>; C]),
}

#[derive(SaveBin, Debug)]
struct TableInner<const R: usize, const C: usize> {
    buf: Box<[[u16; R]; C]>,
}

/// Comparator (descending order) for order keys, making null values greater than every other key.
struct NullsLastReverse<K: ChronologicalKey>(K);

trait ChronologicalKey: Ord + Copy + crate::io::SaveBin {
    fn is_null(&self) -> bool;
}

impl<const R: usize> ChronologicalOrder for ChronologicalList<R> {
    fn cmp_entries(&self, id_a: usize, id_b: usize) -> Ordering {
        NullsLastReverse(self.items.buf[0][id_a]).cmp(&NullsLastReverse(self.items.buf[0][id_b]))
    }

    fn swap(&mut self, id_a: usize, id_b: usize) {
        self.items.buf[0].swap(id_a, id_b);
    }

    fn insert(&mut self, id: usize) {
        // For this table type in particular, the game wraps around and fails
        // during comparisons, i.e. new entries will appear last after `max`
        // overflows.
        self.max = self.max.wrapping_add(1);
        self.items.buf[0][id] = self.max;
    }
}

impl ChronologicalOrder for Inventory {
    fn cmp_entries(&self, id_a: usize, id_b: usize) -> Ordering {
        let id_a: u16 = id_a.try_into().unwrap();
        let id_b: u16 = id_b.try_into().unwrap();
        let (ty_a, ty_b) = (
            ItemType::get_by_item_id(id_a),
            ItemType::get_by_item_id(id_b),
        );
        let (slot_a, slot_b) = (
            self.slots(ty_a)
                .iter()
                .find(|s| s.item_id() == id_a)
                .expect("item a not found"),
            self.slots(ty_b)
                .iter()
                .find(|s| s.item_id() == id_b)
                .expect("item b not found"),
        );
        NullsLastReverse(slot_a.chronological_id())
            .cmp(&NullsLastReverse(slot_b.chronological_id()))
    }

    fn swap(&mut self, id_a: usize, id_b: usize) {
        let id_a: u16 = id_a.try_into().unwrap();
        let id_b: u16 = id_b.try_into().unwrap();

        let (slot_a, slot_b) = self.split_slots_mut(id_a, id_b);

        let b_chrono = slot_b.chronological_id();
        slot_b.set_chronological_id(slot_a.chronological_id());
        slot_a.set_chronological_id(b_chrono);
    }

    fn insert(&mut self, id: usize) {
        let id: u16 = id.try_into().unwrap();
        // Item table will also wrap on overflow.
        let max = self.chronological_id_max.wrapping_add(1);
        self.chronological_id_max = max;
        // Assume item has already been registered.
        let slot = self
            .slots_mut(ItemType::get_by_item_id(id))
            .iter_mut()
            .find(|s| s.item_id() == id)
            .expect("item not yet registered");
        slot.set_chronological_id(max);
    }
}

impl<K: ChronologicalKey> PartialOrd for NullsLastReverse<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: ChronologicalKey> Ord for NullsLastReverse<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 != other.0 {
            if self.0.is_null() {
                return Ordering::Greater;
            }
            if other.0.is_null() {
                return Ordering::Less;
            }
        }
        other.0.cmp(&self.0)
    }
}

impl<K: ChronologicalKey> PartialEq for NullsLastReverse<K> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: ChronologicalKey> Eq for NullsLastReverse<K> {}

impl ChronologicalKey for u16 {
    fn is_null(&self) -> bool {
        *self == 0
    }
}

impl ChronologicalKey for u32 {
    fn is_null(&self) -> bool {
        *self == 0
    }
}
