use crate::sys;

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
    DXIL,
}

impl From<CompileTarget> for sys::SlangCompileTarget {
    fn from(value: CompileTarget) -> Self {
        match value {
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
#[repr(C)]
pub struct ProfileId(pub(crate) u32);

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
