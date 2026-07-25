#![allow(non_snake_case)]

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::{
    AsBoxedComponentType, BoxedComponentType, CapabilityID, CompileTarget, CompilerOptions,
    CompilerPaths, MatrixLayoutMode, Module, PassThrough, ProfileId, SlangContext, TypeConformance,
    TypeConformanceDescriptor, com, sys,
};

/// A global session for interaction with the Slang library.
///
/// An application may create and re-use a single global session across
/// multiple sessions, in order to amortize startups costs (in current
/// Slang this is mostly the cost of loading the Slang standard library).
///
/// A single global session object is currently *not* thread-safe. Unless
/// documented otherwise, a global session and the objects created from it
/// should be externally synchronized when shared across threads. Distinct
/// global sessions may be used from different threads in parallel.
pub struct GlobalSession {
    inner: com::IGlobalSession,
    ctx: SlangContext,
}

impl GlobalSession {
    pub fn new(ctx: SlangContext, paths: CompilerPaths) -> Result<Self> {
        let mut result = None::<com::IGlobalSession>;
        unsafe { (ctx.vtable.create_global_session)(0, &mut result as *mut _ as *mut _).ok() }?;

        let inner = result.context("GlobalSession not created")?;

        let loader = com::ISlangSharedLibraryLoader::from(paths);
        unsafe { inner.setSharedLibraryLoader(&loader) };

        Ok(Self { inner, ctx })
    }

    pub fn context(&self) -> &SlangContext {
        &self.ctx
    }

    pub fn get_build_tag(&self) -> String {
        let s = unsafe { CStr::from_ptr(self.inner.getBuildTagString()) };
        s.to_string_lossy().into_owned()
    }

    pub fn get_compiler_version(&self, pass_through: PassThrough) -> Result<(i32, i32)> {
        let mut out_major = 0;
        let mut out_minor = 0;
        unsafe {
            self.inner.getDownstreamCompilerVersion(
                pass_through.into(),
                &mut out_major,
                &mut out_minor,
            )?;
        }
        Ok((out_major, out_minor))
    }

    pub fn check_compile_target_support(&self, target: CompileTarget) -> Result<()> {
        unsafe { self.inner.checkCompileTargetSupport(target.into())? };
        Ok(())
    }

    pub fn check_pass_through_support(&self, pass_through: PassThrough) -> Result<()> {
        unsafe { self.inner.checkPassThroughSupport(pass_through.into())? };
        Ok(())
    }

    pub fn find_profile(&self, name: &str) -> Option<ProfileId> {
        let name = CString::new(name).ok()?;
        let id = unsafe { self.inner.findProfile(name.as_ptr()) };
        (id != 0).then_some(ProfileId(id))
    }

    pub fn find_capability(&self, name: &str) -> Option<CapabilityID> {
        let name = CString::new(name).ok()?;
        let id = unsafe { self.inner.findProfile(name.as_ptr()) };
        (id != 0).then_some(CapabilityID(id))
    }

    pub fn create_session(&self, desc: &SessionDescriptor) -> Result<Session> {
        Session::new(self, desc)
    }
}

// === Session ===

/// Arguments for [`GlobalSession::create_session`].
#[derive(Default)]
pub struct SessionDescriptor<'a, 's, 'v> {
    /// Search paths used to resolve files referenced by `#include` or `import`.
    pub search_paths: &'a [PathBuf],
    /// Code generation targets to include in the session.
    pub targets: &'a [TargetDescriptor],
    /// A list of pre-defined macros.
    pub preprocessor_macros: &'a [(&'s str, Cow<'v, str>)],
    /// Default layout to assume for variables with matrix types..
    pub default_matrix_layout_mode: MatrixLayoutMode,
    /// A set of compiler options.
    pub options: CompilerOptions,
}

/// A code generation target description.
#[derive(Debug)]
pub struct TargetDescriptor {
    /// The target format to generate code for (e.g., SPIR-V, DXIL, etc.).
    pub format: CompileTarget,
    /// The compilation profile supported by the target.
    ///
    /// The profile can be requested via [`GlobalSession::find_profile`].
    pub profile: ProfileId,
    /// A set of compiler options.
    pub options: CompilerOptions,
}

/// A session provides a scope for code that is loaded.
///
/// A session can be used to load modules of Slang source code
/// and to request target-specific compiled binaries and layout
/// information.
///
/// In order to be able to load code, the session owns a set
/// of active "search paths" for resolving `#include` directives
/// and `import` declarations, as well as a set of global
/// preprocessor definitions that will be used for all code
/// that gets `import`ed in the session.
///
/// If multiple user shaders are loaded in the same session
/// and import the same module (e.g., two source files do `import X`),
/// then there will only be one copy of `X` loaded within the session.
///
/// In order to be able to generate target code, the session
/// owns a list of available compilation targets, which specify
/// code generation options.
///
/// Code loaded and compiled within a session is owned by the session
/// and will remain resident in memory until the session is released.
/// Applications wishing to control the memory usage for compiled
/// and loaded code should use multiple sessions.
#[derive(Clone)]
pub struct Session {
    pub(crate) inner: com::ISession,
    pub(crate) ctx: SlangContext,
}

impl Session {
    pub fn new(global_session: &GlobalSession, desc: &SessionDescriptor) -> Result<Self> {
        let mut targets = Vec::with_capacity(desc.targets.len());
        for target in desc.targets {
            targets.push(sys::slang_TargetDesc {
                structureSize: std::mem::size_of::<sys::slang_TargetDesc>(),
                format: target.format.into(),
                profile: target.profile.0,
                flags: 0,
                floatingPointMode: sys::SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_DEFAULT,
                lineDirectiveMode: sys::SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_DEFAULT,
                forceGLSLScalarBufferLayout: false,
                compilerOptionEntries: target.options.entries.as_ptr(),
                compilerOptionEntryCount: target.options.entries.len() as _,
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

            preprocessor_macros.push(sys::slang_PreprocessorMacroDesc { name, value });
        }

        let native_desc = sys::slang_SessionDesc {
            structureSize: std::mem::size_of::<sys::slang_SessionDesc>(),
            targets: targets.as_ptr(),
            targetCount: targets.len() as _,
            flags: 0,
            defaultMatrixLayoutMode: desc.default_matrix_layout_mode.into(),
            searchPaths: search_path_ptrs.as_ptr(),
            searchPathCount: search_path_ptrs.len() as _,
            preprocessorMacros: preprocessor_macros.as_ptr(),
            preprocessorMacroCount: preprocessor_macros.len() as _,
            fileSystem: std::ptr::null_mut(),
            enableEffectAnnotations: false,
            allowGLSLSyntax: false,
            compilerOptionEntries: desc.options.entries.as_ptr(),
            compilerOptionEntryCount: desc.options.entries.len() as _,
            skipSPIRVValidation: false,
        };

        let mut session = None;
        unsafe {
            global_session
                .inner
                .createSession(&native_desc, &mut session)?
        };

        Ok(Self {
            inner: session.context("session was not created")?,
            ctx: global_session.ctx.clone(),
        })
    }

    pub fn context(&self) -> &SlangContext {
        &self.ctx
    }

    pub fn load_module<T: Into<Vec<u8>>>(
        &self,
        name: &str,
        path: Option<&Path>,
        source: T,
    ) -> Result<Module> {
        Module::new(self, name, path, source.into())
    }

    pub fn combine_component_types<'a, I, T>(&self, items: I) -> Result<BoxedComponentType>
    where
        I: IntoIterator<Item = T, IntoIter: 'a>,
        T: AsBoxedComponentType + 'a,
    {
        let items = items.into_iter();
        let mut component_types = Vec::with_capacity(items.size_hint().0);
        for item in items {
            let boxed = item.as_boxed();
            anyhow::ensure!(
                self.inner == *boxed.session,
                "all components must be from the same `Session`"
            );
            anyhow::ensure!(
                self.ctx == *boxed.ctx,
                "all components must be from the same `SlangContext`"
            );
            component_types.push(boxed.inner.clone());
        }

        let mut diagnostics = None;
        let mut composite_type = None;
        let result = unsafe {
            self.inner.createCompositeComponentType(
                component_types.as_ptr(),
                component_types.len() as _,
                &mut composite_type,
                &mut diagnostics,
            )
        };
        self.ctx.log_diagnostics(diagnostics);
        result?;

        Ok(BoxedComponentType {
            inner: composite_type.context("composite context type was not created")?,
            session: self.inner.clone(),
            ctx: self.ctx.clone(),
        })
    }

    pub fn create_type_conformance(
        &self,
        desc: &TypeConformanceDescriptor<'_>,
    ) -> Result<TypeConformance> {
        TypeConformance::new(self, desc)
    }
}
