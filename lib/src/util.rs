use recordkeeper_macros::SaveBin;

use crate::{error::SaveError, io::SaveBin};

/// Nul-terminated string with fixed storage and maximum length.
///
/// Extra bytes are not guaranteed to be nulls.
#[derive(SaveBin, Debug)]
pub struct FixStr<const MAX: usize> {
    buf: [u8; MAX],
}

/// Dynamic array with fixed capacity.
#[derive(SaveBin, Debug)]
pub struct FixVec<T, const MAX: usize>
where
    for<'a> T: SaveBin<'a>,
    SaveError: for<'a> From<<T as SaveBin<'a>>::ReadError>,
    SaveError: for<'a> From<<T as SaveBin<'a>>::WriteError>,
{
    buf: [T; MAX],
    len: u64,
}

impl<T, const MAX: usize> FixVec<T, MAX>
where
    for<'a> T: SaveBin<'a>,
    SaveError: for<'a> From<<T as SaveBin<'a>>::ReadError>,
    SaveError: for<'a> From<<T as SaveBin<'a>>::WriteError>,
{
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.buf.iter().take(self.len as usize)
    }
}
