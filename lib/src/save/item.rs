use std::marker::PhantomData;
use recordkeeper_macros::SaveBin;

#[derive(SaveBin)]
pub struct ItemSlot {
    item_id: u16,
    amount: u16,

    #[loc(0xf)]
    #[no_getter]
    _padding: PhantomData<()>
}