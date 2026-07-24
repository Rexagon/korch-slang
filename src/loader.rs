#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::{CStr, OsStr, c_char, c_void};
use std::mem::MaybeUninit;
use std::path::PathBuf;

use libloading::{Library, Symbol};
use windows_core::{HRESULT, implement};

use crate::com::{
    ISlangCastable_Impl, ISlangSharedLibrary, ISlangSharedLibrary_Impl, ISlangSharedLibraryLoader,
    ISlangSharedLibraryLoader_Impl,
};
use crate::sys::SlangUUID;

pub struct LoadedLibrary<'a> {
    pub(crate) create_global_session: Symbol<'a, FnCreateGlobalSession>,
}

impl<'a> LoadedLibrary<'a> {
    pub fn new(library: &'a Library) -> Result<Self, libloading::Error> {
        unsafe {
            Ok(Self {
                create_global_session: library
                    .get::<FnCreateGlobalSession>("slang_createGlobalSession")?,
            })
        }
    }
}

type FnCreateGlobalSession = unsafe extern "C" fn(i64, *mut *mut c_void) -> HRESULT;

// === Custom Shared Library Loader ===

#[implement(ISlangSharedLibraryLoader)]
pub struct CustomSharedLibraryLoader {
    pub dxil_path: PathBuf,
    pub dxcompiler_path: PathBuf,
}

impl ISlangSharedLibraryLoader_Impl for CustomSharedLibraryLoader_Impl {
    unsafe fn loadSharedLibrary(
        &self,
        path: *const c_char,
        shared_library_out: *mut MaybeUninit<ISlangSharedLibrary>,
    ) -> HRESULT {
        let path = CStr::from_ptr(path);
        log::debug!("library requested: path={:?}", path.to_str());

        let path = match path.to_bytes() {
            b"dxil" => self.dxil_path.as_os_str(),
            b"dxcompiler" => self.dxcompiler_path.as_os_str(),
            _ => OsStr::from_encoded_bytes_unchecked(path.to_bytes()),
        };

        let library = match Library::new(path) {
            Ok(lib) => lib,
            Err(e) => {
                log::error!("failed to open library: {e:?}");
                return HRESULT(-1);
            }
        };

        debug_assert!(!shared_library_out.is_null());

        let library: ISlangSharedLibrary = CustomSharedLibrary { library }.into();
        *shared_library_out = MaybeUninit::new(library);

        HRESULT(0)
    }
}

impl Drop for CustomSharedLibraryLoader {
    fn drop(&mut self) {
        log::debug!("dropped loader");
    }
}

#[implement(ISlangSharedLibrary)]
pub struct CustomSharedLibrary {
    library: Library,
}

impl ISlangSharedLibrary_Impl for CustomSharedLibrary_Impl {
    unsafe fn findSymbolAddressByName(&self, name: *const c_char) -> *mut c_void {
        let name = CStr::from_ptr(name);
        log::debug!("symbol address requested: name={:?}", name.to_str());

        match self.library.get::<unsafe extern "system" fn()>(name) {
            Ok(symbol) => symbol.into_raw().as_raw_ptr(),
            Err(e) => {
                log::error!("failed to get symbol: name={:?}, {e:?}", name.to_str());
                std::ptr::null_mut()
            }
        }
    }
}

impl ISlangCastable_Impl for CustomSharedLibrary_Impl {
    unsafe fn castAs(&self, guid: &SlangUUID) -> *mut c_void {
        log::debug!(
            "cast requested: 0x{:08x} 0x{:04x} 0x{:04x}",
            guid.data1,
            guid.data2,
            guid.data3
        );

        std::ptr::null_mut()
    }
}

impl Drop for CustomSharedLibrary {
    fn drop(&mut self) {
        log::debug!("dropped shared library");
    }
}
