use crate::sys;

macro_rules! define_enum {
    ($(#[$meta:meta])*
        $name:ident : $name_native:ty, {
        $($variant:ident = $native:ident),*$(,)?
    }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum $name {
            $($variant,)*
        }

        #[allow(non_upper_case_globals)]
        impl $name {
            pub(crate) fn from_slang(value: $name_native) -> Option<Self> {
                use sys::*;
                Some(match value {
                    $($native => Self::$variant,)*
                    _ => return None,
                })
            }
        }

        impl From<$name> for $name_native {
            fn from(value: $name) -> Self {
                use sys::*;
                match value {
                    $($name::$variant => $native,)*
                }
            }
        }
    };
}

// === Type ===

define_enum!(
    /// A kind of type.
    TypeKind : sys::SlangTypeKind, {
    None = SlangTypeKind_SLANG_TYPE_KIND_NONE,
    Struct = SlangTypeKind_SLANG_TYPE_KIND_STRUCT,
    Array = SlangTypeKind_SLANG_TYPE_KIND_ARRAY,
    Matrix = SlangTypeKind_SLANG_TYPE_KIND_MATRIX,
    Vector = SlangTypeKind_SLANG_TYPE_KIND_VECTOR,
    Scalar = SlangTypeKind_SLANG_TYPE_KIND_SCALAR,
    ConstantBuffer = SlangTypeKind_SLANG_TYPE_KIND_CONSTANT_BUFFER,
    Resource = SlangTypeKind_SLANG_TYPE_KIND_RESOURCE,
    SamplerState = SlangTypeKind_SLANG_TYPE_KIND_SAMPLER_STATE,
    TextureBuffer = SlangTypeKind_SLANG_TYPE_KIND_TEXTURE_BUFFER,
    ShaderStorageBuffer = SlangTypeKind_SLANG_TYPE_KIND_SHADER_STORAGE_BUFFER,
    ParameterBlock = SlangTypeKind_SLANG_TYPE_KIND_PARAMETER_BLOCK,
    GenericTypeParameter = SlangTypeKind_SLANG_TYPE_KIND_GENERIC_TYPE_PARAMETER,
    Interface = SlangTypeKind_SLANG_TYPE_KIND_INTERFACE,
    OutputStream = SlangTypeKind_SLANG_TYPE_KIND_OUTPUT_STREAM,
    MeshOutput = SlangTypeKind_SLANG_TYPE_KIND_MESH_OUTPUT,
    Specialized = SlangTypeKind_SLANG_TYPE_KIND_SPECIALIZED,
    Feedback = SlangTypeKind_SLANG_TYPE_KIND_FEEDBACK,
    Pointer = SlangTypeKind_SLANG_TYPE_KIND_POINTER,
    DynamicResource = SlangTypeKind_SLANG_TYPE_KIND_DYNAMIC_RESOURCE,
    Enum = SlangTypeKind_SLANG_TYPE_KIND_ENUM,
});

// === Diagnostic Color ===

/// The color mode for rich diagnostics.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DiagnosticColor {
    #[default]
    Auto,
    Always,
    Never,
}

impl From<DiagnosticColor> for sys::SlangDiagnosticColor {
    fn from(value: DiagnosticColor) -> Self {
        use sys::*;
        match value {
            DiagnosticColor::Auto => SlangDiagnosticColor_SLANG_DIAGNOSTIC_COLOR_AUTO,
            DiagnosticColor::Always => SlangDiagnosticColor_SLANG_DIAGNOSTIC_COLOR_ALWAYS,
            DiagnosticColor::Never => SlangDiagnosticColor_SLANG_DIAGNOSTIC_COLOR_NEVER,
        }
    }
}

// === Optimization Level ===

/// The target code optimization level.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum OptimizationLevel {
    /// Don't optimize at all.
    None,
    /// Default optimization level: balance code quality
    /// and compilation time.
    #[default]
    Default,
    /// Optimize aggressively.
    High,
    /// Include optimizations that may take a very long
    /// time, or may involve severe space-vs-speed tradeoffs.
    Maximal,
}

impl From<OptimizationLevel> for sys::SlangOptimizationLevel {
    fn from(value: OptimizationLevel) -> Self {
        use sys::*;
        match value {
            OptimizationLevel::None => SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_NONE,
            OptimizationLevel::Default => SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_DEFAULT,
            OptimizationLevel::High => SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_HIGH,
            OptimizationLevel::Maximal => SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_MAXIMAL,
        }
    }
}

// === Line Directive Mode ===

/// The line directive mode for output source code.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum LineDirectiveMode {
    /// Default behavior: pick behavior base on target.
    #[default]
    Default,
    /// Don't emit line directives at all.
    None,
    /// Emit standard C-style `#line` directives.
    Standard,
    /// Emit GLSL-style directives with file *number* instead of name.
    GLSL,
    /// Use a source map to track line mappings
    /// (ie no #line will appear in emitting source).
    SourceMap,
}

impl From<LineDirectiveMode> for sys::SlangLineDirectiveMode {
    fn from(value: LineDirectiveMode) -> Self {
        use sys::*;
        match value {
            LineDirectiveMode::Default => SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_DEFAULT,
            LineDirectiveMode::None => SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_NONE,
            LineDirectiveMode::Standard => {
                SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_STANDARD
            }
            LineDirectiveMode::GLSL => SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_GLSL,
            LineDirectiveMode::SourceMap => {
                SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_SOURCE_MAP
            }
        }
    }
}

// === Debug Info Level ===

/// The shader debug information level.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DebugInfoLevel {
    /// Don't emit debug information at all.
    None,
    /// Emit as little debug information as possible,
    /// while still supporting stack trackers.
    Minimal,
    /// Emit whatever is the standard level of debug
    /// information for each target.
    #[default]
    Standard,
    /// Emit as much debug information as possible for each target.
    Maximal,
}

impl From<DebugInfoLevel> for sys::SlangDebugInfoLevel {
    fn from(value: DebugInfoLevel) -> Self {
        use sys::*;
        match value {
            DebugInfoLevel::None => SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_NONE,
            DebugInfoLevel::Minimal => SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_MINIMAL,
            DebugInfoLevel::Standard => SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_STANDARD,
            DebugInfoLevel::Maximal => SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_MAXIMAL,
        }
    }
}

// === FP Mode ===

/// The mode to use for floating-point operations on the target.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FloatingPointMode {
    #[default]
    Default,
    Fast,
    Precise,
}

impl From<FloatingPointMode> for sys::SlangFloatingPointMode {
    fn from(value: FloatingPointMode) -> Self {
        use sys::*;
        match value {
            FloatingPointMode::Default => SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_DEFAULT,
            FloatingPointMode::Fast => SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_FAST,
            FloatingPointMode::Precise => SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_PRECISE,
        }
    }
}

// === Matrix Mode ===

/// The layout to assume for variables with matrix types.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MatrixLayoutMode {
    #[default]
    RowMajor,
    ColumnMajor,
}

impl From<MatrixLayoutMode> for sys::SlangMatrixLayoutMode {
    fn from(value: MatrixLayoutMode) -> Self {
        use sys::*;
        match value {
            MatrixLayoutMode::RowMajor => SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_ROW_MAJOR,
            MatrixLayoutMode::ColumnMajor => SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_COLUMN_MAJOR,
        }
    }
}

// === Target Stuff ===

/// The target format to generate code for.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CompileTarget {
    GLSL,
    HLSL,
    SpirV,
    SpirVAsm,
    DXIL,
    DXILAsm,
}

impl From<CompileTarget> for sys::SlangCompileTarget {
    fn from(value: CompileTarget) -> Self {
        match value {
            CompileTarget::GLSL => sys::SlangCompileTarget_SLANG_GLSL,
            CompileTarget::HLSL => sys::SlangCompileTarget_SLANG_HLSL,
            CompileTarget::SpirV => sys::SlangCompileTarget_SLANG_SPIRV,
            CompileTarget::SpirVAsm => sys::SlangCompileTarget_SLANG_SPIRV_ASM,
            CompileTarget::DXIL => sys::SlangCompileTarget_SLANG_DXIL,
            CompileTarget::DXILAsm => sys::SlangCompileTarget_SLANG_DXIL_ASM,
        }
    }
}

/// An underlying compiler.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PassThrough {
    DXC,
    Glslang,
}

impl From<PassThrough> for sys::SlangPassThrough {
    fn from(value: PassThrough) -> Self {
        use sys::*;
        match value {
            PassThrough::DXC => SlangPassThrough_SLANG_PASS_THROUGH_DXC,
            PassThrough::Glslang => SlangPassThrough_SLANG_PASS_THROUGH_GLSLANG,
        }
    }
}

/// The internal ID of a target profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ProfileId(pub(crate) u32);

/// The internal ID of a capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CapabilityID(pub(crate) u32);

// === Language ===

/// The language of a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SourceLanguage {
    Slang,
    HLSL,
    GLSL,
    SPRIV,
}

impl From<SourceLanguage> for sys::SlangSourceLanguage {
    fn from(value: SourceLanguage) -> Self {
        use sys::*;
        match value {
            SourceLanguage::Slang => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_SLANG,
            SourceLanguage::HLSL => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_HLSL,
            SourceLanguage::GLSL => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_GLSL,
            SourceLanguage::SPRIV => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_SPIRV,
        }
    }
}

// === Shader Stage ===

define_enum!(
    /// A shader stage.
    Stage : sys::SlangStage, {
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
