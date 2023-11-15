mod error;
pub mod io;
mod save;
mod system;
pub mod util;

use crate::error::SaveError;
use std::alloc::Layout;
use std::any::{Any, TypeId};
use std::mem::MaybeUninit;

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
        // Allocate directly on the heap. The `SaveData` struct is *big*. I encountered
        // stack overflow problems in tests.
        let mut save: Box<MaybeUninit<SaveData>> = {
            let layout = Layout::new::<MaybeUninit<SaveData>>();
            assert_ne!(0, layout.size());

            // SAFETY: size > 0
            let ptr = unsafe { std::alloc::alloc(layout) };
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }

            // SAFETY: same behavior as Box::try_new_uninit_in
            unsafe { Box::from_raw(ptr.cast()) }
        };

        // SAFETY: SaveBin::read_into needs to hold the invariant to never read or drop
        // the output pointer.
        unsafe {
            SaveData::read_into(bytes, save.as_mut_ptr())?;
        }

        // Based on currently unstable Box::assume_init, issue 63291.
        // Also make sure that the Box is using the Global alloc, as it's also what
        // Box::from_raw is using. Note that the incubating API handles this automatically.
        assert_eq!(save.type_id(), TypeId::of::<Box<MaybeUninit<SaveData>>>());
        let save = {
            let raw = Box::into_raw(save);
            // SAFETY: only condition is to have fully initialized data, which it is if
            // all reads have succeeded.
            unsafe { Box::from_raw(raw as *mut SaveData) }
        };

        Ok(Self {
            save,
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
