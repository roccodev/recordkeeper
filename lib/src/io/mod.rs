use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::Infallible;
use std::io::Cursor;
use std::marker::PhantomData;
use std::mem::MaybeUninit;

pub trait SaveBin<'src>: Sized {
    type Error;

    fn read(bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error>;

    fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

impl<'src> SaveBin<'src> for u64 {
    type Error = std::io::Error;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
        bytes.read_u64::<LittleEndian>()
    }
}

impl<'src> SaveBin<'src> for f32 {
    type Error = std::io::Error;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
        bytes.read_f32::<LittleEndian>()
    }
}

impl<'src> SaveBin<'src> for u32 {
    type Error = std::io::Error;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
        bytes.read_u32::<LittleEndian>()
    }
}

impl<'src> SaveBin<'src> for u16 {
    type Error = std::io::Error;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
        bytes.read_u16::<LittleEndian>()
    }
}

impl<'src> SaveBin<'src> for u8 {
    type Error = std::io::Error;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
        bytes.read_u8()
    }
}

impl<'src> SaveBin<'src> for bool {
    type Error = std::io::Error;

    fn read(bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
        Ok(u8::read(bytes)? != 0)
    }
}

impl<'src, T> SaveBin<'src> for PhantomData<T> {
    type Error = Infallible;

    fn read(_: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
        Ok(PhantomData)
    }
}

impl<'src, T, const N: usize> SaveBin<'src> for [T; N]
where
    T: SaveBin<'src>,
{
    type Error = T::Error;

    fn read(mut bytes: Cursor<&'src [u8]>) -> Result<Self, Self::Error> {
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

    fn size() -> usize {
        T::size() * N
    }
}
