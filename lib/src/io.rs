use crate::error::SaveError;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use std::convert::Infallible;
use std::io::Cursor;
use std::marker::PhantomData;
use std::ops::Deref;

/// Provides support for reading structs from save files, and writing to portions of them.
///
/// ## Derive macro
/// The recommended way to implement this trait for a struct is to use the `#[derive(SaveBin)]`
/// attribute.
///
/// The example below also showcases the optional `loc`, `assert`, and `size` attributes.
/// ```ignore
/// use recordkeeper::SaveBin;
///
/// #[derive(SaveBin)]
/// // We can provide a size hint manually. If the actual size is bigger,
/// // reads will panic. If it is smaller, extra space will be accounted
/// // for when reading or writing.
/// #[size(32)]
/// struct Position {
///     // fields must be of types that implement SaveBin
///     x: f32,
///     y: f32,
///     z: f32,
///
///     // We don't know what the other fields are, but
///     // we can skip them by forcing an offset. The
///     // offset is relative to the start of the struct.
///     #[loc(0x10)]
///     yaw: f32,
///     pitch: f32,
///     
///     // We observed that this field is always 0, so we can
///     // add an assertion in case this ever changes
///     #[assert(0)]
///     _unknown: u32,
/// }
/// ```
///
/// Failed assertions on struct fields will not panic, they will instead cause reads to throw
/// an [`AssertionError`].  
/// It is also possible to throw a custom error, like in this example:
/// ```ignore
/// use recordkeeper::SaveBin;
///
/// struct UnsupportedVersion;
///
/// #[derive(SaveBin)]
/// struct Version {
///     // version, must be 1.
///     #[assert(1, UnsupportedVersion)]
///     version: u32,
/// }
/// ```
///
/// To construct errors with custom values, the `ACTUAL` keyword may be used in place of the
/// encountered value, e.g.
/// ```ignore
/// #[assert(1, CustomErrorWithValue(ACTUAL))]
/// ```
///
/// [`AssertionError`]: crate::error::SaveError::AssertionError
pub trait SaveBin: Sized {
    type ReadError;
    type WriteError;

    /// Reads the type from a byte buffer, into the given memory buffer.
    ///
    /// The cursor's internal marker will be modified after the call,
    /// even if the read fails.
    ///
    /// ## Alternatives
    /// If the type also implements `Default` and `Copy`, the [`read`] function from the
    /// [`OwnedSaveBin`] trait may be used instead.
    ///
    /// [`read`]: OwnedSaveBin::read
    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError>;

    /// Writes the type to a byte buffer.
    ///
    /// ## Panics
    /// This function may panic if there isn't enough space to write the data.
    /// When writing save files, the old save file data should be used as the base.
    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError>;

    /// Returns the total size of this type *when serialized into the save binary format*.
    fn size() -> usize {
        std::mem::size_of::<Self>()
    }
}

pub trait OwnedSaveBin: SaveBin {
    /// Reads the type from a byte buffer.
    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError>;
}

macro_rules! byteorder_impl {
    ($($types:tt ) *) => {
        $(
            impl SaveBin for $types {
                type ReadError = std::io::Error;
                type WriteError = std::convert::Infallible;

                fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
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

impl SaveBin for u8 {
    type ReadError = std::io::Error;
    type WriteError = SaveError;

    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        bytes.read_u8()
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        let pos = bytes.get_mut(0).ok_or(SaveError::UnexpectedEof)?;
        *pos = *self;
        Ok(())
    }
}

impl SaveBin for i8 {
    type ReadError = std::io::Error;
    type WriteError = SaveError;

    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        bytes.read_i8()
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        (*self as u8).write(bytes)
    }
}

impl SaveBin for bool {
    type ReadError = std::io::Error;
    type WriteError = SaveError;

    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        let value = <u8 as OwnedSaveBin>::read(bytes)?;
        Ok(value != 0)
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        u8::from(*self).write(bytes)
    }
}

impl<T> SaveBin for PhantomData<T> {
    type ReadError = Infallible;
    type WriteError = Infallible;

    fn read(_: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        Ok(Self)
    }

    fn write(&self, _: &mut [u8]) -> Result<(), Self::WriteError> {
        Ok(())
    }
}

impl<T, const N: usize> SaveBin for [T; N]
where
    T: SaveBin + std::fmt::Debug,
    T::ReadError: Into<SaveError>,
{
    type ReadError = SaveError;
    type WriteError = T::WriteError;

    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        let mut items = Vec::new();
        for _ in 0..N {
            let item = T::read(bytes).map_err(Into::into)?;
            items.push(item);
        }
        // TODO: better to default initialize the array?
        Ok(items.try_into().unwrap())
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
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

impl<T: SaveBin> SaveBin for Box<T> {
    type ReadError = T::ReadError;

    type WriteError = T::WriteError;

    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        T::read(bytes).map(Box::new)
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        self.deref().write(bytes)
    }
}

impl<T> OwnedSaveBin for T
where
    T: SaveBin + Default + Copy,
{
    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        Self::read(bytes)
    }
}
