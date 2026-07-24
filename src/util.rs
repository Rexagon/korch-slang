use windows_core::implement;

use crate::com::{ISlangBlob, ISlangBlob_Impl};

#[implement(ISlangBlob)]
pub struct Blob {
    inner: Vec<u8>,
}

impl Blob {
    pub fn new<T: Into<Vec<u8>>>(value: T) -> Self {
        Self {
            inner: value.into(),
        }
    }
}

impl ISlangBlob_Impl for Blob_Impl {
    unsafe fn getBufferPointer(&self) -> *const std::ffi::c_void {
        self.inner.as_ptr().cast()
    }

    unsafe fn getBufferSize(&self) -> usize {
        self.inner.len()
    }
}
