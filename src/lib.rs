#![doc = include_str!("../README.md")]

pub use crate::component_type::{
    AsBoxedComponentType, BoxedComponentType, BoxedComponentTypeRef, ComponentLayout, EntryPoint,
    EntryPointLayout, EntryPointsIter, EntryPointsLayoutIter, LinkedModule, Module,
    SpecializationArg, TypeConformance, TypeConformanceDescriptor, TypeLayout,
};
pub use crate::context::SlangContext;
pub use crate::env::CompilerPaths;
pub use crate::options::CompilerOptions;
pub use crate::session::{GlobalSession, Session, SessionDescriptor, TargetDescriptor};
pub use crate::types::{
    CapabilityID, CompileTarget, DebugInfoLevel, DiagnosticColor, FloatingPointMode,
    LineDirectiveMode, MatrixLayoutMode, OptimizationLevel, PassThrough, ProfileId, SourceLanguage,
    Stage, TypeKind,
};

mod com;
mod component_type;
mod context;
mod env;
mod options;
mod session;
mod sys;
mod types;
mod util;
