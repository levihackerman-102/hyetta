use std::ffi::c_void;
use std::fs::File;
use std::num::NonZeroUsize;
use std::os::fd::AsFd;
use std::ptr::NonNull;

use nix::sys::mman::{mmap, munmap, MapFlags, ProtFlags};

use crate::error::AppError;

/// A read-only, zero-copy view of a file's contents, mapped into memory
/// via `mmap(2)`. The mapping is torn down in `Drop`, so the byte slice
/// returned by `as_bytes` can never outlive the `MappedFile` that owns it.
pub struct MappedFile {
    _file: File,
    mapping: Option<NonNull<c_void>>,
    len: usize,
}

impl MappedFile {
    pub fn open(path: &str) -> Result<Self, AppError> {
        let file = File::open(path)?;
        let len = file.metadata()?.len() as usize;

        // mmap() of a zero-length region is undefined behavior on POSIX,
        // so skip the syscall entirely and represent it as an empty view.
        if len == 0 {
            return Ok(Self {
                _file: file,
                mapping: None,
                len: 0,
            });
        }

        let length = NonZeroUsize::new(len).expect("len == 0 handled above");
        let mapping = unsafe {
            mmap(
                None,
                length,
                ProtFlags::PROT_READ,
                MapFlags::MAP_PRIVATE,
                file.as_fd(),
                0,
            )
        }?;

        Ok(Self {
            _file: file,
            mapping: Some(mapping),
            len,
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self.mapping {
            Some(ptr) => unsafe { std::slice::from_raw_parts(ptr.as_ptr() as *const u8, self.len) },
            None => &[],
        }
    }
}

impl Drop for MappedFile {
    fn drop(&mut self) {
        if let Some(ptr) = self.mapping.take() {
            // Best-effort: a failed munmap here only leaks the mapping,
            // it can't corrupt memory, so it isn't worth panicking over.
            let _ = unsafe { munmap(ptr, self.len) };
        }
    }
}
