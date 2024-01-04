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
        let value = <u8 as SaveBin>::read(bytes)?;
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
    T: SaveBin,
    T::ReadError: Into<SaveError>,
{
    type ReadError = SaveError;
    type WriteError = T::WriteError;

    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        array_init::try_array_init(|_| T::read(bytes)).map_err(Into::into)
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

impl<T: SaveBin, const N: usize> SaveBin for Box<[T; N]>
where
    SaveError: From<<T as SaveBin>::ReadError>,
    SaveError: From<<T as SaveBin>::WriteError>,
{
    type ReadError = T::ReadError;

    type WriteError = T::WriteError;

    fn read(bytes: &mut Cursor<&[u8]>) -> Result<Self, Self::ReadError> {
        // Read into a vec first to avoid large stack allocations with Box::new.
        let mut items = Vec::with_capacity(N);
        for _ in 0..N {
            let item = T::read(bytes)?;
            items.push(item);
        }
        // Unreachable since we return early if we do not successfully read N elements.
        match items.into_boxed_slice().try_into() {
            Ok(items) => Ok(items),
            Err(_) => unreachable!(),
        }
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        let values: &[T; N] = &*self;
        values.write(bytes)
    }

    fn size() -> usize {
        T::size() * N
    }
}
