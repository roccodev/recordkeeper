use recordkeeper_macros::SaveBin;
use std::marker::PhantomData;

#[derive(SaveBin)]
#[size(16)]
pub struct ItemSlot {
    item_id: u16,
    amount: u16,
}
