// === Custom Shared Library Loader ===

use std::ffi::{CStr, OsStr, c_char, c_void};
use std::path::PathBuf;

use libloading::Library;
use windows_core::{OutRef, implement};

use crate::com::{
    ISlangCastable_Impl, ISlangSharedLibrary, ISlangSharedLibrary_Impl, ISlangSharedLibraryLoader,
    ISlangSharedLibraryLoader_Impl,
};
use crate::sys::SlangUUID;

macro_rules! define_paths {
    ($ident:ident, { $($path:ident = $name:literal),*$(,)? }) => {
        #[derive(Default, Clone)]
        pub struct $ident {
            $(pub $path: Option<PathBuf>,)*
        }

        impl $ident {
            fn resolve_by_name<'a>(&'a self, path: &'a CStr) -> &'a OsStr {
                match path.to_bytes() {
                    $($name => {
                        if let Some(path) = &self.$path {
                            return path.as_os_str();
                        }
                    },)*
                    _ => {}
                }

                unsafe { OsStr::from_encoded_bytes_unchecked(path.to_bytes()) }
            }
        }
    };
}

define_paths!(CompilerPaths, {
    dxil = b"dxil",
    dxcompiler = b"dxcompiler",
});

impl From<CompilerPaths> for ISlangSharedLibraryLoader {
    fn from(paths: CompilerPaths) -> Self {
        CustomSharedLibraryLoader { paths }.into()
    }
}

#[implement(ISlangSharedLibraryLoader)]
struct CustomSharedLibraryLoader {
    paths: CompilerPaths,
}

impl ISlangSharedLibraryLoader_Impl for CustomSharedLibraryLoader_Impl {
    unsafe fn loadSharedLibrary(
        &self,
        path: *const c_char,
        shared_library_out: OutRef<ISlangSharedLibrary>,
    ) -> windows_core::Result<()> {
        let path = unsafe { CStr::from_ptr(path) };
        log::debug!("library requested: path={:?}", path.to_str());

        let path = self.paths.resolve_by_name(path);
        let library = match unsafe { Library::new(path) } {
            Ok(lib) => lib,
            Err(e) => {
                log::error!("failed to open library: {e:?}");
                return Err(windows_core::Error::empty());
            }
        };

        let library: ISlangSharedLibrary = CustomSharedLibrary { library }.into();
        shared_library_out.write(Some(library))
    }
}

impl Drop for CustomSharedLibraryLoader {
    fn drop(&mut self) {
        log::debug!("dropped loader");
    }
}

#[implement(ISlangSharedLibrary)]
struct CustomSharedLibrary {
    library: Library,
}

impl ISlangSharedLibrary_Impl for CustomSharedLibrary_Impl {
    unsafe fn findSymbolAddressByName(&self, name: *const c_char) -> *mut c_void {
        let name = unsafe { CStr::from_ptr(name) };
        log::debug!("symbol address requested: name={:?}", name.to_str());

        match unsafe { self.library.get::<unsafe extern "system" fn()>(name) } {
            Ok(symbol) => unsafe { symbol.into_raw() }.as_raw_ptr(),
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
