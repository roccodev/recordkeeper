mod error;
pub mod io;
mod save;
mod system;
pub mod util;

use std::io::Cursor;

use crate::error::SaveError;

use crate::io::SaveBin;
pub use save::*;
pub use system::*;

pub type SaveResult<T> = Result<T, SaveError>;

pub type SaveFile = FileBuffer<SaveData>;
pub type SystemFile = FileBuffer<SystemData>;

pub struct FileBuffer<T> {
    bytes: Box<[u8]>,
    parsed: Box<T>,
}

pub enum DataFile {
    Save(SaveFile),
    System(SystemFile),
}

impl DataFile {
    /// Reads a save or system file from a slice, and allocates it on the heap.
    /// Both the given buffer and the parsed save file will be allocated.
    ///
    /// * If the file is a save file (usually starts with `bf3game` or `bf3dlc`),
    /// a [`DataFile::Save`] is returned.
    /// * If the file is a system file (the name usually starts with `bf3system`),
    /// a [`DataFile::System`] is returned.
    ///
    /// Otherwise, if the file could not be recognized, an error is returned.
    ///
    /// On success, you can extract either the save file or the system file by
    /// matching on the result:
    /// ```
    /// # use recordkeeper::{DataFile, SaveResult};
    /// #
    /// # fn read(bytes: &[u8]) -> SaveResult<()> {
    ///     let data = DataFile::from_bytes(bytes)?;
    ///     match data {
    ///         DataFile::Save(save) => { /* This is a save file */ },
    ///         DataFile::System(sys) => { /* This is the system file */ }
    ///     }
    ///     # Ok(())
    /// # }
    /// ```
    pub fn from_bytes(bytes: &[u8]) -> SaveResult<Self> {
        if bytes.len() < 4 {
            return Err(SaveError::UnexpectedEof);
        }
        let magic: [u8; 4] = bytes[0..4].try_into().unwrap();
        match magic {
            SAVE_MAGIC => SaveFile::from_bytes(bytes).map(Self::Save),
            SYSTEM_MAGIC => SystemFile::from_bytes(bytes).map(Self::System),
            _ => Err(SaveError::UnrecognizedFormat),
        }
    }

    pub fn is_save(&self) -> bool {
        matches!(self, Self::Save(_))
    }

    pub fn is_system(&self) -> bool {
        matches!(self, Self::System(_))
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            DataFile::Save(s) => s.bytes(),
            DataFile::System(s) => s.bytes(),
        }
    }

    pub fn write(&mut self) -> SaveResult<()> {
        match self {
            DataFile::Save(s) => s.write(),
            DataFile::System(s) => s.write(),
        }
    }
}

impl<T> FileBuffer<T> {
    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    pub fn save(&self) -> &T {
        &self.parsed
    }

    pub fn save_mut(&mut self) -> &mut T {
        &mut self.parsed
    }
}

impl SaveFile {
    /// Reads a save file from a slice, and allocates it on the heap.
    ///
    /// Both the given buffer and the parsed save file will be allocated.
    pub fn from_bytes(bytes: &[u8]) -> SaveResult<Self> {
        let mut reader = Cursor::new(bytes);
        let save = SaveData::read(&mut reader)?;

        Ok(Self {
            parsed: Box::new(save),
            bytes: Box::from(bytes),
        })
    }

    pub fn write(&mut self) -> SaveResult<()> {
        self.parsed.write(self.bytes.as_mut())
    }
}

impl SystemFile {
    /// Reads a system file from a slice, and allocates it on the heap.
    ///
    /// Both the given buffer and the parsed save file will be allocated.
    pub fn from_bytes(bytes: &[u8]) -> SaveResult<Self> {
        let mut reader = Cursor::new(bytes);
        let save = SystemData::read(&mut reader)?;

        Ok(Self {
            parsed: Box::new(save),
            bytes: Box::from(bytes),
        })
    }

    pub fn write(&mut self) -> SaveResult<()> {
        self.parsed.write(self.bytes.as_mut())
    }
}
