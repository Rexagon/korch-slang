use crate::sys;

macro_rules! define_enum {
    ($name:ident : $name_native:ty, {
        $($variant:ident = $native:ident),*$(,)?
    }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

define_enum!(TypeKind : sys::SlangTypeKind, {
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptimizationLevel {
    None,
    #[default]
    Default,
    High,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineDirectiveMode {
    #[default]
    Default,
    None,
    Standard,
    Glsl,
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
            LineDirectiveMode::Glsl => SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_GLSL,
            LineDirectiveMode::SourceMap => {
                SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_SOURCE_MAP
            }
        }
    }
}

// === Debug Info Level ===

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugInfoLevel {
    None,
    Minimal,
    #[default]
    Standard,
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy)]
pub enum CompileTarget {
    HLSL,
    DXIL,
}

impl From<CompileTarget> for sys::SlangCompileTarget {
    fn from(value: CompileTarget) -> Self {
        match value {
            CompileTarget::HLSL => sys::SlangCompileTarget_SLANG_HLSL,
            CompileTarget::DXIL => sys::SlangCompileTarget_SLANG_DXIL,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PassThrough {
    DXC,
}

impl From<PassThrough> for sys::SlangPassThrough {
    fn from(value: PassThrough) -> Self {
        use sys::*;
        match value {
            PassThrough::DXC => SlangPassThrough_SLANG_PASS_THROUGH_DXC,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ProfileId(pub(crate) u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct CapabilityID(pub(crate) u32);

// === Language ===

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    Slang,
    HLSL,
}

impl From<SourceLanguage> for sys::SlangSourceLanguage {
    fn from(value: SourceLanguage) -> Self {
        use sys::*;
        match value {
            SourceLanguage::Slang => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_SLANG,
            SourceLanguage::HLSL => SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_HLSL,
        }
    }
}

// === Shader Stage ===

define_enum!(Stage : sys::SlangStage, {
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
