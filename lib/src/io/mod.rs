use crate::error::SaveError;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::convert::Infallible;
use std::io::Cursor;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

pub trait SaveBin<'src>: Sized {
    type ReadError;
    type WriteError;

    fn read(bytes: Cursor<&'src [u8]>) -> Result<Self, Self::ReadError>;

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError>;

    fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

macro_rules! byteorder_impl {
    ($($types:tt ) *) => {
        $(
            impl<'src> SaveBin<'src> for $types {
                type ReadError = std::io::Error;
                type WriteError = std::convert::Infallible;

                fn read(mut bytes: std::io::Cursor<&'src [u8]>) -> Result<Self, Self::ReadError> {
                    paste::paste! { bytes.[<read_ $types >]::<LittleEndian>() }
                }

                fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
                    paste::paste! { LittleEndian::[<write_ $types >](bytes, *self) }
                    Ok(())
                }
            }
        )*
    };
}

byteorder_impl!(u64 i64 f64 u32 i32 f32 u16 i16);

impl<'src> SaveBin<'src> for u8 {
    type ReadError = std::io::Error;
    type WriteError = SaveError;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::ReadError> {
        bytes.read_u8()
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        let pos = bytes.get_mut(0).ok_or(SaveError::UnexpectedEof)?;
        *pos = *self;
        Ok(())
    }
}

impl<'src> SaveBin<'src> for i8 {
    type ReadError = std::io::Error;
    type WriteError = SaveError;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::ReadError> {
        bytes.read_i8()
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        (*self as u8).write(bytes)
    }
}

impl<'src> SaveBin<'src> for bool {
    type ReadError = std::io::Error;
    type WriteError = SaveError;

    fn read(bytes: Cursor<&'src [u8]>) -> Result<Self, Self::ReadError> {
        Ok(u8::read(bytes)? != 0)
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        u8::from(*self).write(bytes)
    }
}

impl<'src, T> SaveBin<'src> for PhantomData<T> {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn read(_: Cursor<&'src [u8]>) -> Result<Self, Self::ReadError> {
        Ok(PhantomData)
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        Ok(())
    }
}

impl<'src, T, const N: usize> SaveBin<'src> for [T; N]
where
    T: SaveBin<'src>,
{
    type ReadError = T::ReadError;
    type WriteError = T::WriteError;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::ReadError> {
        let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        // Loop is drop-safe, see MaybeUninit docs
        for elem in &mut data[..] {
            let item = T::read(bytes.clone())?;
            elem.write(item);
            let size: u64 = T::size().try_into().expect("size too large");
            bytes.set_position(bytes.position() + size);
        }

        // https://github.com/rust-lang/rust/issues/61956
        let ptr = &data as *const _ as *const [T; N];
        std::mem::forget(data);
        Ok(unsafe { ptr.read() })
    }

    fn write(&self, mut bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        let mut pos = 0;
        let item_size = T::size();

        for item in self {
            let cur_bytes = &mut bytes[pos..];
            item.write(cur_bytes)?;
            pos += item_size;
        }

        Ok(())
    }

    fn size() -> usize {
        T::size() * N
    }
}
