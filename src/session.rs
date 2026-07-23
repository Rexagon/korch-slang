#![allow(non_snake_case)]

use std::ffi::CStr;

use anyhow::{Context, Result};
use windows_core::Interface;

use crate::LoadedLibrary;
use crate::com::{IGlobalSession, ISlangSharedLibraryLoader};
use crate::sys::root::{
    SlangCompileTarget_SLANG_DXIL, SlangCompileTargetIntegral,
    SlangPassThrough_SLANG_PASS_THROUGH_DXC, SlangPassThroughIntegral,
};

pub struct GlobalSession(IGlobalSession);

impl GlobalSession {
    pub fn new(lib: &LoadedLibrary<'_>) -> Result<Self> {
        let mut result = None::<IGlobalSession>;
        unsafe { (lib.create_global_session)(0, &mut result as *mut _ as *mut _).ok() }?;

        let result = result.context("GlobalSession was not created")?;

        Ok(Self(result))
    }

    pub fn get_build_tag(&self) -> String {
        let s = unsafe { CStr::from_ptr(self.0.getBuildTagString()) };
        s.to_string_lossy().into_owned()
    }

    pub fn get_compiler_version(&mut self, pass_through: PassThrough) -> Result<(i32, i32)> {
        let mut out_major = 0;
        let mut out_minor = 0;
        unsafe {
            self.0
                .getDownstreamCompilerVersion(pass_through.into(), &mut out_major, &mut out_minor)
                .ok()?;
        }
        Ok((out_major, out_minor))
    }

    pub fn check_compile_target_support(&self, target: CompileTarget) -> Result<()> {
        unsafe { self.0.checkCompileTargetSupport(target.into()).ok()? };
        Ok(())
    }

    pub fn check_pass_through_support(&self, pass_through: PassThrough) -> Result<()> {
        unsafe { self.0.checkPassThroughSupport(pass_through.into()).ok()? };
        Ok(())
    }

    pub fn set_library_loader(&self, loader: Option<&ISlangSharedLibraryLoader>) {
        unsafe { self.0.setSharedLibraryLoader(loader) };
    }

    pub fn get_library_loader(&self) -> Option<ISlangSharedLibraryLoader> {
        let raw = unsafe { self.0.getSharedLibraryLoader() };
        unsafe { ISlangSharedLibraryLoader::from_raw_borrowed(&raw) }.cloned()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompileTarget {
    DXIL,
}

impl From<CompileTarget> for SlangCompileTargetIntegral {
    fn from(value: CompileTarget) -> Self {
        match value {
            CompileTarget::DXIL => SlangCompileTarget_SLANG_DXIL,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PassThrough {
    DXC,
}

impl From<PassThrough> for SlangPassThroughIntegral {
    fn from(value: PassThrough) -> Self {
        match value {
            PassThrough::DXC => SlangPassThrough_SLANG_PASS_THROUGH_DXC,
        }
    }
}
