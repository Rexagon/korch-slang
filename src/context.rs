use std::ffi::{c_char, c_void};
use std::path::Path;
use std::rc::Rc;

use anyhow::{Context, Result};
use libloading::Library;
#[cfg(unix)]
use libloading::os::unix::Symbol;
#[cfg(windows)]
use libloading::os::windows::Symbol;
use windows_core::HRESULT;

use crate::sys::{
    SlangProgramLayout, SlangReflectionEntryPoint, SlangStage, SlangUInt, slang_FunctionReflection,
};
use crate::{CompilerPaths, GlobalSession, com};

#[derive(Clone)]
pub struct SlangContext {
    pub(crate) vtable: Rc<SlangVtable>,
}

impl SlangContext {
    pub fn new(slang_path: impl AsRef<Path>) -> Result<Self> {
        let library = unsafe { libloading::Library::new(slang_path.as_ref()) }
            .context("failed to open Slang compiler library")?;

        Self::from_library(library)
    }

    pub fn from_library(library: Library) -> Result<Self> {
        Ok(Self {
            vtable: Rc::new(SlangVtable::new(library)?),
        })
    }

    pub fn create_global_session(&self, paths: CompilerPaths) -> Result<GlobalSession> {
        GlobalSession::new(self.clone(), paths)
    }

    // TODO: Store some writer in the context.
    pub(crate) fn log_diagnostics(&self, diagnostics: Option<com::ISlangBlob>) {
        let Some(diagnostics) = diagnostics else {
            return;
        };

        let data = unsafe {
            std::slice::from_raw_parts(
                diagnostics.getBufferPointer().cast::<u8>(),
                diagnostics.getBufferSize(),
            )
        };
        let Ok(s) = std::str::from_utf8(data) else {
            log::error!("failed to parse diagnostics");
            return;
        };

        eprintln!("{s}");
    }
}

impl Eq for SlangContext {}
impl PartialEq for SlangContext {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.vtable, &other.vtable)
    }
}

macro_rules! define_vtable {
    ($ident:ident, {
        $($fn:ident: {
            name: $fn_name:literal,
            ty: $fn_type:ty$(,)?
        }),*$(,)?
    }$(,)?) => {
        pub(crate) struct SlangVtable {
            #[expect(unused)]
            pub library: Library,
            $(
                #[allow(unused)]
                pub $fn: Symbol<$fn_type>,
            )*
        }

        impl SlangVtable {
            fn new(library: Library) -> Result<Self, libloading::Error> {
                Ok(Self {
                    $($fn: unsafe { library.get::<$fn_type>($fn_name)?.into_raw() },)*
                    library,
                })
            }
        }
    };
}

define_vtable!(SlangVtable, {
    create_global_session: {
        name: "slang_createGlobalSession",
        ty: unsafe extern "C" fn(i64, *mut *mut c_void) -> HRESULT,
    },
    reflection_get_entry_point_count: {
        name: "spReflection_getEntryPointCount",
        ty: unsafe extern "C" fn(*mut SlangProgramLayout) -> SlangUInt,
    },
    reflection_get_entry_point_by_index: {
        name: "spReflection_getEntryPointByIndex",
        ty: unsafe extern "C" fn(*mut SlangProgramLayout, SlangUInt) -> *mut SlangReflectionEntryPoint,
    },
    reflection_fn_get_name: {
        name: "spReflectionFunction_GetName",
        ty: unsafe extern "C" fn(*mut slang_FunctionReflection) -> *const c_char,
    },
    reflection_entry_point_get_name: {
        name: "spReflectionEntryPoint_getName",
        ty: unsafe extern "C" fn(*mut SlangReflectionEntryPoint) -> *const c_char,
    },
    reflection_entry_point_get_stage: {
        name: "spReflectionEntryPoint_getStage",
        ty: unsafe extern "C" fn(*mut SlangReflectionEntryPoint) -> SlangStage,
    }
});
