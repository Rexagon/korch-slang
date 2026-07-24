#![allow(non_snake_case)]

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};
use std::ptr::NonNull;

use anyhow::{Context, Result};
use windows_core::Interface;

use crate::LoadedLibrary;
use crate::com::{
    IEntryPoint, IGlobalSession, IModule, ISession, ISlangBlob, ISlangSharedLibraryLoader,
};
use crate::sys::{
    SlangCompileTarget_SLANG_DXIL, SlangCompileTargetIntegral,
    SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_DEFAULT,
    SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_DEFAULT, SlangMatrixLayoutMode,
    SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_COLUMN_MAJOR,
    SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_ROW_MAJOR, SlangPassThrough_SLANG_PASS_THROUGH_DXC,
    SlangPassThroughIntegral, SlangProgramLayout, SlangSourceLanguage,
    SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_HLSL,
    SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_SLANG, SlangStage,
    SlangStage_SLANG_STAGE_AMPLIFICATION, SlangStage_SLANG_STAGE_ANY_HIT,
    SlangStage_SLANG_STAGE_CALLABLE, SlangStage_SLANG_STAGE_CLOSEST_HIT,
    SlangStage_SLANG_STAGE_COMPUTE, SlangStage_SLANG_STAGE_DISPATCH, SlangStage_SLANG_STAGE_DOMAIN,
    SlangStage_SLANG_STAGE_GEOMETRY, SlangStage_SLANG_STAGE_HULL,
    SlangStage_SLANG_STAGE_INTERSECTION, SlangStage_SLANG_STAGE_MESH, SlangStage_SLANG_STAGE_MISS,
    SlangStage_SLANG_STAGE_NODE, SlangStage_SLANG_STAGE_PIXEL,
    SlangStage_SLANG_STAGE_RAY_GENERATION, SlangStage_SLANG_STAGE_VERTEX, slang_FunctionReflection,
    slang_PreprocessorMacroDesc, slang_SessionDesc, slang_TargetDesc,
};
use crate::util::Blob;

pub struct GlobalSession {
    inner: IGlobalSession,
}

impl GlobalSession {
    pub fn new(lib: &LoadedLibrary<'_>) -> Result<Self> {
        let mut result = None::<IGlobalSession>;
        unsafe { (lib.create_global_session)(0, &mut result as *mut _ as *mut _).ok() }?;

        let inner = result.context("GlobalSession was not created")?;

        Ok(Self { inner })
    }

    pub fn get_build_tag(&self) -> String {
        let s = unsafe { CStr::from_ptr(self.inner.getBuildTagString()) };
        s.to_string_lossy().into_owned()
    }

    pub fn get_compiler_version(&mut self, pass_through: PassThrough) -> Result<(i32, i32)> {
        let mut out_major = 0;
        let mut out_minor = 0;
        unsafe {
            self.inner
                .getDownstreamCompilerVersion(pass_through.into(), &mut out_major, &mut out_minor)
                .ok()?;
        }
        Ok((out_major, out_minor))
    }

    pub fn check_compile_target_support(&self, target: CompileTarget) -> Result<()> {
        unsafe { self.inner.checkCompileTargetSupport(target.into()).ok()? };
        Ok(())
    }

    pub fn check_pass_through_support(&self, pass_through: PassThrough) -> Result<()> {
        unsafe {
            self.inner
                .checkPassThroughSupport(pass_through.into())
                .ok()?
        };
        Ok(())
    }

    pub fn set_library_loader(&self, loader: Option<&ISlangSharedLibraryLoader>) {
        unsafe { self.inner.setSharedLibraryLoader(loader) };
    }

    pub fn get_library_loader(&self) -> Option<ISlangSharedLibraryLoader> {
        let raw = unsafe { self.inner.getSharedLibraryLoader() };
        unsafe { ISlangSharedLibraryLoader::from_raw_borrowed(&raw) }.cloned()
    }

    pub fn find_profile(&self, name: &str) -> Option<ProfileId> {
        let name = CString::new(name).ok()?;
        let id = unsafe { self.inner.findProfile(name.as_ptr()) };
        (id != 0).then_some(ProfileId(id))
    }

    pub fn create_session(&mut self, desc: &SessionDescriptor) -> Result<Session> {
        let mut targets = Vec::with_capacity(desc.targets.len());
        for target in desc.targets {
            targets.push(slang_TargetDesc {
                structureSize: std::mem::size_of::<slang_TargetDesc>(),
                format: target.format.into(),
                profile: target.profile.0,
                flags: 0,
                floatingPointMode: SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_DEFAULT,
                lineDirectiveMode: SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_DEFAULT,
                forceGLSLScalarBufferLayout: false,
                compilerOptionEntries: std::ptr::null(),
                compilerOptionEntryCount: 0,
            });
        }

        let mut strings =
            Vec::with_capacity(desc.search_paths.len() + desc.preprocessor_macros.len() * 2);

        let mut search_path_ptrs = Vec::with_capacity(desc.search_paths.len());
        for path in desc.search_paths {
            let path = strings.push_mut(
                CString::new(path.as_os_str().as_encoded_bytes()).context("invalid search path")?,
            );
            search_path_ptrs.push(path.as_ptr());
        }

        let mut preprocessor_macros = Vec::with_capacity(desc.preprocessor_macros.len());
        for (name, value) in desc.preprocessor_macros {
            let name = strings
                .push_mut(CString::new(*name).context("invalid macros name")?)
                .as_ptr();
            let value = strings
                .push_mut(CString::new(value.as_bytes()).context("invalid macros value")?)
                .as_ptr();

            preprocessor_macros.push(slang_PreprocessorMacroDesc { name, value });
        }

        let native_desc = slang_SessionDesc {
            structureSize: std::mem::size_of::<slang_SessionDesc>(),
            targets: targets.as_ptr(),
            targetCount: targets.len() as _,
            flags: 0,
            defaultMatrixLayoutMode: SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_ROW_MAJOR,
            searchPaths: search_path_ptrs.as_ptr(),
            searchPathCount: search_path_ptrs.len() as _,
            preprocessorMacros: preprocessor_macros.as_ptr(),
            preprocessorMacroCount: preprocessor_macros.len() as _,
            fileSystem: std::ptr::null_mut(),
            enableEffectAnnotations: false,
            allowGLSLSyntax: false,
            compilerOptionEntries: std::ptr::null(),
            compilerOptionEntryCount: 0,
            skipSPIRVValidation: false,
        };

        let mut session = None;
        unsafe { self.inner.createSession(&native_desc, &mut session).ok()? };

        Ok(Session {
            inner: session.context("session was not created")?,
        })
    }
}

// === Session ===

#[derive(Default)]
pub struct SessionDescriptor<'a, 's, 'v> {
    pub search_paths: &'a [PathBuf],
    pub targets: &'a [TargetDescriptor],
    pub preprocessor_macros: &'a [(&'s str, Cow<'v, str>)],
    pub default_matrix_layout_mode: MatrixLayoutMode,
}

pub struct Session {
    inner: ISession,
}

impl Session {
    pub fn load_module<T: Into<Vec<u8>>>(
        &mut self,
        name: &str,
        path: Option<&Path>,
        blob: T,
    ) -> Result<Module> {
        let name = CString::new(name).context("invalid module name")?;
        let path = path
            .map(|path| CString::new(path.as_os_str().as_encoded_bytes()))
            .transpose()
            .context("invalid module path")?;

        let blob: ISlangBlob = Blob::new(blob).into();

        let mut diagnostics = None;
        let module = unsafe {
            self.inner.loadModuleFromSource(
                name.as_ptr(),
                path.as_ref().map(|p| p.as_ptr()).unwrap_or_default(),
                &blob,
                &mut diagnostics,
            )
        };
        log_diagnostics(diagnostics);

        let module =
            unsafe { IModule::from_raw_borrowed(&module) }.context("module was not created")?;

        Ok(Module {
            inner: module.clone(),
        })
    }
}

pub struct Module {
    inner: IModule,
}

impl Module {
    pub fn entry_points_iter(&self) -> EntryPointsIter<'_> {
        let count = self.entry_point_count();
        EntryPointsIter {
            module: self,
            next_index: 0,
            count,
        }
    }

    pub fn entry_point_count(&self) -> usize {
        unsafe { self.inner.getDefinedEntryPointCount() as usize }
    }

    pub fn get_entry_point(&self, index: usize) -> Option<EntryPoint> {
        if index > i32::MAX as usize {
            return None;
        }

        let mut entry_point = None;
        let res = unsafe {
            self.inner
                .getDefinedEntryPoint(index as i32, &mut entry_point)
        };
        if res.is_err() {
            return None;
        }

        entry_point.map(|inner| EntryPoint { inner })
    }
}

#[derive(Clone)]
pub struct EntryPointsIter<'a> {
    module: &'a Module,
    next_index: usize,
    count: usize,
}

impl Iterator for EntryPointsIter<'_> {
    type Item = EntryPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_index >= self.count {
            return None;
        }

        let index = self.next_index;
        self.next_index += 1;
        self.module.get_entry_point(index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for EntryPointsIter<'_> {
    fn len(&self) -> usize {
        self.count.saturating_sub(self.next_index)
    }
}

pub struct EntryPoint {
    inner: IEntryPoint,
}

impl EntryPoint {
    pub fn get_name(&self, lib: &LoadedLibrary<'_>) -> Result<String> {
        let r = self.get_reflection();
        let name = unsafe { (lib.reflection_fn_get_name)(r) };
        anyhow::ensure!(!name.is_null(), "failed to get function name");

        let name = unsafe { CStr::from_ptr(name) };
        name.to_str()
            .map(ToOwned::to_owned)
            .context("invalid function name")
    }

    pub fn get_stage(&self, lib: &LoadedLibrary<'_>) -> Result<Stage> {
        let layout = self.get_layout()?;
        let entry_point_count = unsafe { (lib.reflection_get_entry_point_count)(layout.as_ptr()) };
        anyhow::ensure!(entry_point_count == 1, "invalid component layout");

        let entry_point = unsafe { (lib.reflection_get_entry_point_by_index)(layout.as_ptr(), 0) };
        let stage = unsafe { (lib.reflection_entry_point_get_stage)(entry_point) };
        Stage::from_slang(stage).context("unknown stage")
    }

    fn get_reflection(&self) -> *mut slang_FunctionReflection {
        unsafe { self.inner.getFunctionReflection() }
    }

    fn get_layout(&self) -> Result<NonNull<SlangProgramLayout>> {
        let mut diagnostics = None;
        let layout = unsafe { self.inner.getLayout(0, &mut diagnostics) };
        log_diagnostics(diagnostics);
        NonNull::new(layout).context("failed to get layout")
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatrixLayoutMode {
    #[default]
    RowMajor,
    ColumnMajor,
}

impl From<MatrixLayoutMode> for SlangMatrixLayoutMode {
    fn from(value: MatrixLayoutMode) -> Self {
        match value {
            MatrixLayoutMode::RowMajor => SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_ROW_MAJOR,
            MatrixLayoutMode::ColumnMajor => SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_COLUMN_MAJOR,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TargetDescriptor {
    pub format: CompileTarget,
    pub profile: ProfileId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ProfileId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    Slang,
    HLSL,
}

impl From<SourceLanguage> for SlangSourceLanguage {
    fn from(value: SourceLanguage) -> Self {
        match value {
            SourceLanguage::Slang => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_SLANG,
            SourceLanguage::HLSL => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_HLSL,
        }
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

macro_rules! define_enum {
    ($name:ident : $name_native:ident, {
        $($variant:ident = $native:ident),*$(,)?
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            fn from_slang(value: $name_native) -> Option<Self> {
                Some(match value {
                    $($native => Self::$variant,)*
                    _ => return None,
                })
            }
        }

        impl From<$name> for $name_native {
            fn from(value: $name) -> Self {
                match value {
                    $($name::$variant => $native,)*
                }
            }
        }
    };
}

define_enum!(Stage : SlangStage, {
    Vertex = SlangStage_SLANG_STAGE_VERTEX,
    Hull = SlangStage_SLANG_STAGE_HULL,
    Domain = SlangStage_SLANG_STAGE_DOMAIN,
    Geometry = SlangStage_SLANG_STAGE_GEOMETRY,
    Pixel = SlangStage_SLANG_STAGE_PIXEL,
    Compute = SlangStage_SLANG_STAGE_COMPUTE,
    RayGeneration = SlangStage_SLANG_STAGE_RAY_GENERATION,
    Intersection = SlangStage_SLANG_STAGE_INTERSECTION,
    AnyHit = SlangStage_SLANG_STAGE_ANY_HIT,
    ClosestHit = SlangStage_SLANG_STAGE_CLOSEST_HIT,
    Miss = SlangStage_SLANG_STAGE_MISS,
    Callable = SlangStage_SLANG_STAGE_CALLABLE,
    Mesh = SlangStage_SLANG_STAGE_MESH,
    Amplification = SlangStage_SLANG_STAGE_AMPLIFICATION,
    Dispatch = SlangStage_SLANG_STAGE_DISPATCH,
    Node = SlangStage_SLANG_STAGE_NODE,
});

fn log_diagnostics(diagnostics: Option<ISlangBlob>) {
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

    log::warn!("{s}");
}
