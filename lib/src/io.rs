use crate::error::SaveError;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};
use std::convert::Infallible;
use std::io::Cursor;
use std::marker::PhantomData;

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
pub trait SaveBin<'src>: Sized {
    type ReadError;
    type WriteError;

    /// Reads the type from a byte buffer, into the given memory buffer.
    ///
    /// The cursor's internal marker will be modified after the call,
    /// even if the read fails.
    ///
    /// ## Safety
    /// This function is safe to call provided that `out` points to valid memory. It's not required
    /// for that memory to be initialized, therefore implementations **must not read or drop
    /// any part of the output memory**, and `out` **must point to initialized memory if the read
    /// succeeds**.
    ///
    /// If the read fails, the state of the output memory is undefined.
    ///
    /// ## Alternatives
    /// If the type also implements `Default` and `Copy`, the [`read`] function from the
    /// [`OwnedSaveBin`] trait may be used instead.
    ///
    /// [`read`]: OwnedSaveBin::read
    unsafe fn read_into(bytes: &'src [u8], out: *mut Self) -> Result<(), Self::ReadError>;

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

pub trait OwnedSaveBin<'src>: SaveBin<'src> {
    /// Reads the type from a byte buffer.
    fn read(bytes: &'src [u8]) -> Result<Self, Self::ReadError>;
}

macro_rules! byteorder_impl {
    ($($types:tt ) *) => {
        $(
            impl<'src> SaveBin<'src> for $types {
                type ReadError = std::io::Error;
                type WriteError = std::convert::Infallible;

                unsafe fn read_into(mut bytes: &'src [u8], out: *mut Self) -> Result<(), Self::ReadError> {
                    // Integer types don't implement Drop, so no need for ptr::write
                    *out = paste::paste! { bytes.[<read_ $types >]::<LittleEndian>()? };
                    Ok(())
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

    unsafe fn read_into(mut bytes: &'src [u8], out: *mut Self) -> Result<(), Self::ReadError> {
        // Integer types don't implement Drop, so no need for ptr::write
        *out = bytes.read_u8()?;
        Ok(())
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

    unsafe fn read_into(mut bytes: &'src [u8], out: *mut Self) -> Result<(), Self::ReadError> {
        *out = bytes.read_i8()?;
        Ok(())
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        (*self as u8).write(bytes)
    }
}

impl<'src> SaveBin<'src> for bool {
    type ReadError = std::io::Error;
    type WriteError = SaveError;

    unsafe fn read_into(bytes: &'src [u8], out: *mut Self) -> Result<(), Self::ReadError> {
        *out = <u8 as OwnedSaveBin>::read(bytes)? != 0;
        Ok(())
    }

    fn write(&self, bytes: &mut [u8]) -> Result<(), Self::WriteError> {
        u8::from(*self).write(bytes)
    }
}

impl<'src, T> SaveBin<'src> for PhantomData<T> {
    type ReadError = Infallible;
    type WriteError = Infallible;

    unsafe fn read_into(_: &'src [u8], _: *mut Self) -> Result<(), Self::ReadError> {
        Ok(())
    }

    fn write(&self, _: &mut [u8]) -> Result<(), Self::WriteError> {
        Ok(())
    }
}

impl<'src, T, const N: usize> SaveBin<'src> for [T; N]
where
    T: SaveBin<'src>,
    T::ReadError: Into<SaveError>,
{
    type ReadError = SaveError;
    type WriteError = T::WriteError;

    unsafe fn read_into(bytes: &'src [u8], out: *mut Self) -> Result<(), Self::ReadError> {
        let size = T::size();
        if size == 0 {
            return Ok(());
        }
        if bytes.len() < size {
            return Err(SaveError::UnexpectedEof);
        }
        let mut out: *mut T = out.cast();
        for i in (0..N * size).step_by(size) {
            T::read_into(&bytes[i..i + size], out).map_err(Into::into)?;
            out = out.add(1);
        }
        Ok(())
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

impl<'src, T> OwnedSaveBin<'src> for T
where
    T: SaveBin<'src> + Default + Copy,
{
    fn read(bytes: &'src [u8]) -> Result<Self, Self::ReadError> {
        let mut out = Self::default();
        // SAFETY: the previous value is perfectly valid, and does not implement Drop
        unsafe {
            Self::read_into(bytes, &mut out as *mut _)?;
        }
        Ok(out)
    }
}
