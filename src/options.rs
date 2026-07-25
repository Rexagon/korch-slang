use std::ffi::{CStr, CString, c_char};
use std::path::Path;

use crate::{
    CapabilityID, CompileTarget, DebugInfoLevel, DiagnosticColor, FloatingPointMode,
    LineDirectiveMode, OptimizationLevel, ProfileId, Stage, sys,
};

/// A set of Slang or pass-through compiler options.
///
/// NOTE: Some options are related to the Slang compiler,
/// and must be specified in [`SessionDescriptor::options`]. And
/// some options are for the pass-through compiler
/// and must be specified in [`TargetDescriptor::options`].
///
/// [`SessionDescriptor::options`]: crate::SessionDescriptor::options
/// [`TargetDescriptor::options`]: crate::SessionDescriptor::options
#[derive(Default)]
pub struct CompilerOptions {
    pub(crate) strings: Vec<CString>,
    pub(crate) entries: Vec<sys::slang_CompilerOptionEntry>,
}

impl CompilerOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn macro_define(self, key: impl Into<Vec<u8>>, value: impl Into<Vec<u8>>) -> Self {
        self.push_str2(sys::slang_CompilerOptionName_MacroDefine, key, value)
    }

    pub fn include(self, path: impl AsRef<Path>) -> Self {
        self.push_str1(
            sys::slang_CompilerOptionName_Include,
            path.as_ref().as_os_str().as_encoded_bytes(),
        )
    }

    pub fn matrix_layout_column(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_MatrixLayoutColumn,
            enable as _,
            0,
        )
    }

    pub fn matrix_layout_row(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_MatrixLayoutRow,
            enable as _,
            0,
        )
    }

    pub fn profile(self, profile: ProfileId) -> Self {
        self.push_ints(sys::slang_CompilerOptionName_Profile, profile.0 as _, 0)
    }

    pub fn stage(self, stage: Stage) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_Stage,
            sys::SlangStage::from(stage) as _,
            0,
        )
    }

    pub fn target(self, target: CompileTarget) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_Target,
            sys::SlangCompileTarget::from(target) as _,
            0,
        )
    }

    pub fn warnings_as_errors(self, filter: impl AsRef<str>) -> Self {
        self.push_str1(
            sys::slang_CompilerOptionName_WarningsAsErrors,
            filter.as_ref(),
        )
    }

    pub fn disable_warnings(self, filter: impl AsRef<str>) -> Self {
        self.push_str1(
            sys::slang_CompilerOptionName_DisableWarnings,
            filter.as_ref(),
        )
    }

    pub fn enable_warning(self, code: impl AsRef<str>) -> Self {
        self.push_str1(sys::slang_CompilerOptionName_EnableWarning, code.as_ref())
    }

    pub fn disable_warning(self, code: impl AsRef<str>) -> Self {
        self.push_str1(sys::slang_CompilerOptionName_DisableWarning, code.as_ref())
    }

    pub fn report_downstream_time(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_ReportDownstreamTime,
            enable as _,
            0,
        )
    }

    pub fn report_perf_benchmark(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_ReportPerfBenchmark,
            enable as _,
            0,
        )
    }

    pub fn skip_spirv_validation(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_SkipSPIRVValidation,
            enable as _,
            0,
        )
    }

    pub fn capability(self, capability: CapabilityID) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_Capability,
            capability.0 as _,
            0,
        )
    }

    pub fn default_image_format_unknown(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_DefaultImageFormatUnknown,
            enable as _,
            0,
        )
    }

    pub fn disable_dynamic_dispatch(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_DisableDynamicDispatch,
            enable as _,
            0,
        )
    }

    pub fn disable_specialization(self, value: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_DisableSpecialization,
            value as _,
            0,
        )
    }

    pub fn floating_point_mode(self, mode: FloatingPointMode) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_DisableSpecialization,
            sys::SlangFloatingPointMode::from(mode) as _,
            0,
        )
    }

    pub fn debug_information(self, level: DebugInfoLevel) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_DebugInformation,
            sys::SlangDebugInfoLevel::from(level) as _,
            0,
        )
    }

    pub fn line_directive_mode(self, mode: LineDirectiveMode) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_LineDirectiveMode,
            sys::SlangLineDirectiveMode::from(mode) as _,
            0,
        )
    }

    pub fn optimization(self, level: OptimizationLevel) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_Optimization,
            sys::SlangOptimizationLevel::from(level) as _,
            0,
        )
    }

    pub fn obfuscate(self, enable: bool) -> Self {
        self.push_ints(sys::slang_CompilerOptionName_Obfuscate, enable as _, 0)
    }

    pub fn vulkan_use_entry_point_name(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_VulkanUseEntryPointName,
            enable as _,
            0,
        )
    }

    pub fn glsl_force_scalar_layout(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_GLSLForceScalarLayout,
            enable as _,
            0,
        )
    }

    pub fn emit_spirv_directly(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_EmitSpirvDirectly,
            enable as _,
            0,
        )
    }

    pub fn no_code_gen(self, enable: bool) -> Self {
        self.push_ints(sys::slang_CompilerOptionName_NoCodeGen, enable as _, 0)
    }

    pub fn diagnostic_color(self, color: DiagnosticColor) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_DiagnosticColor,
            sys::SlangDiagnosticColor::from(color) as _,
            0,
        )
    }

    pub fn no_mangle(self, enable: bool) -> Self {
        self.push_ints(sys::slang_CompilerOptionName_NoMangle, enable as _, 0)
    }

    pub fn validate_uniformity(self, enable: bool) -> Self {
        self.push_ints(
            sys::slang_CompilerOptionName_ValidateUniformity,
            enable as _,
            0,
        )
    }

    // === Helpers ===

    fn push_ints(mut self, name: sys::slang_CompilerOptionName, i0: i32, i1: i32) -> Self {
        self.entries.push(sys::slang_CompilerOptionEntry {
            name,
            value: sys::slang_CompilerOptionValue {
                kind: sys::slang_CompilerOptionValueKind_Int,
                intValue0: i0,
                intValue1: i1,
                stringValue0: std::ptr::null(),
                stringValue1: std::ptr::null(),
            },
        });
        self
    }

    fn push_strings(
        mut self,
        name: sys::slang_CompilerOptionName,
        s0: *const c_char,
        s1: *const c_char,
    ) -> Self {
        self.entries.push(sys::slang_CompilerOptionEntry {
            name,
            value: sys::slang_CompilerOptionValue {
                kind: sys::slang_CompilerOptionValueKind_String,
                intValue0: 0,
                intValue1: 0,
                stringValue0: s0,
                stringValue1: s1,
            },
        });
        self
    }

    fn push_str1<T: Into<Vec<u8>>>(mut self, name: sys::slang_CompilerOptionName, s0: T) -> Self {
        let s0 = self.strings.push_mut(CString::new(s0).unwrap()).as_ptr();
        self.push_strings(name, s0, std::ptr::null())
    }

    fn push_str2<S0: Into<Vec<u8>>, S1: Into<Vec<u8>>>(
        mut self,
        name: sys::slang_CompilerOptionName,
        s0: S0,
        s1: S1,
    ) -> Self {
        let s0 = self.strings.push_mut(CString::new(s0).unwrap()).as_ptr();
        let s1 = self.strings.push_mut(CString::new(s1).unwrap()).as_ptr();
        self.push_strings(name, s0, s1)
    }
}

impl std::fmt::Debug for CompilerOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn map_str<'a>(s: *const c_char) -> Option<&'a CStr> {
            if s.is_null() {
                None
            } else {
                Some(unsafe { CStr::from_ptr(s) })
            }
        }

        #[allow(unused)]
        #[derive(Debug)]
        struct Entry<'a> {
            name: i32,
            value: Value<'a>,
        }

        struct Value<'a>(&'a sys::slang_CompilerOptionValue);

        impl std::fmt::Debug for Value<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.0.kind {
                    sys::slang_CompilerOptionValueKind_Int => f
                        .debug_list()
                        .entry(&self.0.intValue0)
                        .entry(&self.0.intValue1)
                        .finish(),
                    _ => f
                        .debug_list()
                        .entry(&map_str(self.0.stringValue0))
                        .entry(&map_str(self.0.stringValue1))
                        .finish(),
                }
            }
        }

        let mut list = f.debug_list();
        for option in &self.entries {
            list.entry(&Entry {
                name: option.name,
                value: Value(&option.value),
            });
        }
        list.finish()
    }
}
