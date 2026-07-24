use std::ffi::{CStr, CString, c_char};
use std::path::Path;

use anyhow::{Context, Result};
use windows_core::implement;

use crate::com::{ISlangBlob, ISlangBlob_Impl};

pub(crate) fn to_ffi_string(str: impl Into<Vec<u8>>) -> Result<CString> {
    CString::new(str).context("cannot pass null in string")
}

pub(crate) fn to_ffi_path(path: impl AsRef<Path>) -> Result<CString> {
    CString::new(path.as_ref().as_os_str().as_encoded_bytes()).context("cannot pass null in path")
}

pub(crate) fn from_ffi_string(str: *const c_char) -> Result<String> {
    if !str.is_null() {
        let name = unsafe { CStr::from_ptr(str) };
        Ok(name.to_str()?.to_owned())
    } else {
        Err(anyhow::anyhow!("got null pointer as string"))
    }
}

impl From<Vec<u8>> for ISlangBlob {
    fn from(value: Vec<u8>) -> Self {
        Blob { value }.into()
    }
}

#[implement(ISlangBlob)]
struct Blob {
    value: Vec<u8>,
}

impl ISlangBlob_Impl for Blob_Impl {
    unsafe fn getBufferPointer(&self) -> *const std::ffi::c_void {
        self.value.as_ptr().cast()
    }

    unsafe fn getBufferSize(&self) -> usize {
        self.value.len()
    }
}
