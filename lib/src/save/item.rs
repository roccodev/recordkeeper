use recordkeeper_macros::SaveBin;
use std::marker::PhantomData;

#[derive(SaveBin)]
pub struct ItemSlot {
    item_id: u16,
    amount: u16,

    #[loc(0xf)]
    _padding: PhantomData<()>,
}
