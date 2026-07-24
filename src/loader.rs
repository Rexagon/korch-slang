#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::ffi::{CStr, OsStr, c_char, c_void};
use std::path::PathBuf;

use libloading::{Library, Symbol};
use windows_core::{HRESULT, OutRef, implement};

use crate::com::{
    ISlangCastable_Impl, ISlangSharedLibrary, ISlangSharedLibrary_Impl, ISlangSharedLibraryLoader,
    ISlangSharedLibraryLoader_Impl,
};
use crate::sys::{
    SlangProgramLayout, SlangReflectionEntryPoint, SlangStage, SlangUInt, SlangUUID,
    slang_FunctionReflection,
};

pub struct LoadedLibrary<'a> {
    pub(crate) create_global_session: Symbol<'a, FnCreateGlobalSession>,

    pub(crate) reflection_get_entry_point_count: Symbol<'a, FnReflectionGetEntryPointCount>,
    pub(crate) reflection_get_entry_point_by_index: Symbol<'a, FnReflectionGetEntryPointByIndex>,

    pub(crate) reflection_fn_get_name: Symbol<'a, FnReflectionFnGetName>,

    pub(crate) reflection_entry_point_get_name: Symbol<'a, FnReflectionEntryPointGetName>,
    pub(crate) reflection_entry_point_get_stage: Symbol<'a, FnReflectionEntryPointGetStage>,
}

impl<'a> LoadedLibrary<'a> {
    pub fn new(library: &'a Library) -> Result<Self, libloading::Error> {
        unsafe {
            Ok(Self {
                create_global_session: library.get("slang_createGlobalSession")?,
                reflection_get_entry_point_count: library.get("spReflection_getEntryPointCount")?,
                reflection_get_entry_point_by_index: library
                    .get("spReflection_getEntryPointByIndex")?,
                reflection_fn_get_name: library.get("spReflectionFunction_GetName")?,
                reflection_entry_point_get_name: library.get("spReflectionEntryPoint_getName")?,
                reflection_entry_point_get_stage: library.get("spReflectionEntryPoint_getStage")?,
            })
        }
    }
}

type FnCreateGlobalSession = unsafe extern "C" fn(i64, *mut *mut c_void) -> HRESULT;
type FnReflectionGetEntryPointCount = unsafe extern "C" fn(*mut SlangProgramLayout) -> SlangUInt;
type FnReflectionGetEntryPointByIndex =
    unsafe extern "C" fn(*mut SlangProgramLayout, SlangUInt) -> *mut SlangReflectionEntryPoint;
type FnReflectionFnGetName = unsafe extern "C" fn(*mut slang_FunctionReflection) -> *const c_char;
type FnReflectionEntryPointGetName =
    unsafe extern "C" fn(*mut SlangReflectionEntryPoint) -> *const c_char;
type FnReflectionEntryPointGetStage =
    unsafe extern "C" fn(*mut SlangReflectionEntryPoint) -> SlangStage;

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
        shared_library_out: OutRef<ISlangSharedLibrary>,
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

        let library: ISlangSharedLibrary = CustomSharedLibrary { library }.into();
        match shared_library_out.write(Some(library)) {
            Ok(()) => HRESULT(0),
            Err(e) => e.into(),
        }
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
