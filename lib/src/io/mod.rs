use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::Infallible;
use std::io::Cursor;

pub trait SaveBin<'src>: Sized {
    type Error;

    fn read(bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error>;

    fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}
