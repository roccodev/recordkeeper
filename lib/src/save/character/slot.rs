pub struct Slot<N>(pub(crate) N);

pub struct SlotMut<'a, N>(pub(crate) &'a mut N);

pub trait EmptySlot {
    fn is_empty(&self) -> bool;
}

pub trait EmptySlotMut: EmptySlot {
    fn set_empty(&mut self);
}

impl<N> Slot<N>
where
    Self: EmptySlot,
    N: Copy,
{
    /// Returns the current value, or [`None`] if the slot is empty.
    pub fn get(&self) -> Option<N> {
        (!self.is_empty()).then(|| self.0)
    }
}

impl<'a, N> SlotMut<'a, N>
where
    Self: EmptySlotMut,
    N: Copy,
{
    /// Returns the current value, or [`None`] if the slot is empty.
    pub fn get(&self) -> Option<N> {
        (!self.is_empty()).then(|| *self.0)
    }

    /// Updates the current value. Accepts [`Some`] for a valid entry
    /// and [`None`] for an empty slot.
    pub fn set(&mut self, value: Option<N>) {
        match value {
            Some(n) => *self.0 = n,
            None => self.set_empty(),
        }
    }
}
