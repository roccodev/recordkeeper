mod error;
pub mod io;
mod save;
mod system;
pub mod util;

use std::io::Cursor;

use crate::error::SaveError;

use crate::io::SaveBin;
pub use save::*;

pub type SaveResult<T> = Result<T, SaveError>;

pub struct SaveFile {
    bytes: Box<[u8]>,
    save: Box<SaveData>,
}

impl SaveFile {
    /// Reads a save file from a slice, and allocates it on the heap.
    ///
    /// Both the given buffer and the parsed save file will be allocated.
    pub fn from_bytes(bytes: &[u8]) -> SaveResult<Self> {
        let mut reader = Cursor::new(bytes);
        let save = SaveData::read(&mut reader)?;

        Ok(Self {
            save: Box::new(save),
            bytes: Box::from(bytes),
        })
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes.as_ref()
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.bytes.as_mut()
    }

    pub fn save(&self) -> &SaveData {
        &self.save
    }

    pub fn save_mut(&mut self) -> &mut SaveData {
        &mut self.save
    }

    pub fn write(&mut self) -> SaveResult<()> {
        self.save.write(self.bytes.as_mut())
    }
}
