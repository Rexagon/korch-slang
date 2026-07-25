#![allow(non_snake_case)]

use std::ffi::{c_char, c_void};

use windows_core::{IUnknown, IUnknown_Vtbl, OutRef, Ref, Result, interface};

use crate::sys::{
    SlangCapabilityID, SlangCompileRequest, SlangCompileTarget, SlangInt, SlangInt32,
    SlangLayoutRules, SlangParameterCategory, SlangPassThrough, SlangPassThroughIntegral,
    SlangProfileID, SlangProgramLayout, SlangReflectionType, SlangStage, SlangUInt, SlangUUID,
    slang_CompilerOptionEntry, slang_ContainerType, slang_DeclReflection, slang_FunctionReflection,
    slang_SessionDesc, slang_SourceLocation, slang_SpecializationArg, slang_TypeLayoutReflection,
};

#[interface("c140b5fd-0c78-452e-ba7c-1a1e70c7f71c")]
pub unsafe trait IGlobalSession: IUnknown {
    pub fn createSession(
        &self,
        desc: *const slang_SessionDesc,
        out_session: OutRef<ISession>,
    ) -> Result<()>;
    pub fn findProfile(&self, name: *const c_char) -> SlangProfileID;

    fn stub_setDownstreamCompilerPath(&self);
    fn stub_setDownstreamCompilerPrelude(&self);
    fn stub_getDownstreamCompilerPrelude(&self);

    pub fn getBuildTagString(&self) -> *const c_char;

    fn stub_setDefaultDownstreamCompiler(&self);
    fn stub_getDefaultDownstreamCompiler(&self);
    fn stub_setLanguagePrelude(&self);
    fn stub_getLanguagePrelude(&self);

    pub fn createCompileRequest(
        &self,
        out_compile_request: *mut *mut SlangCompileRequest,
    ) -> Result<()>;

    fn stub_addBuiltins(&self);

    pub fn setSharedLibraryLoader(&self, loader: Ref<ISlangSharedLibraryLoader>);
    pub fn getSharedLibraryLoader(&self) -> *mut c_void;

    pub fn checkCompileTargetSupport(&self, target: SlangCompileTarget) -> Result<()>;
    pub fn checkPassThroughSupport(&self, pass_through: SlangPassThrough) -> Result<()>;

    fn stub_compileCoreModule(&self);
    fn stub_loadCoreModule(&self);
    fn stub_saveCoreModule(&self);

    pub fn findCapability(&self, name: *const c_char) -> SlangCapabilityID;

    fn stub_setDownstreamCompilerForTransition(&self);
    fn stub_getDownstreamCompilerForTransition(&self);
    fn stub_getCompilerElapsedTime(&self);
    fn stub_setSPIRVCoreGrammar(&self);
    fn stub_parseCommandLineArguments(&self);
    fn stub_getSessionDescDigest(&self);
    fn stub_compileBuiltinModule(&self);
    fn stub_loadBuiltinModule(&self);
    fn stub_saveBuiltinModule(&self);

    // NOTE: Can't call this for some reason.
    pub fn getDownstreamCompilerVersion(
        &self,
        pass_through: SlangPassThroughIntegral,
        out_major: *mut i32,
        out_minor: *mut i32,
    ) -> Result<()>;
}

#[interface("67618701-d116-468f-ab3b-474bedce0e3d")]
pub unsafe trait ISession: IUnknown {
    /// Returns IGlobalSession.
    pub fn getGlobalSession(&self) -> *mut c_void;
    /// Returns IModule.
    pub fn loadModule(
        &self,
        module_name: *const c_char,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;
    /// Returns IModule.
    pub fn loadModuleFromSource(
        &self,
        module_name: *const c_char,
        path: *const c_char,
        source: Ref<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;
    pub fn createCompositeComponentType(
        &self,
        component_types: *const IComponentType,
        component_type_count: SlangInt,
        out_composite_component_type: OutRef<IComponentType>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;
    pub fn specializeType(
        &self,
        type_reflection: *mut SlangReflectionType,
        specialization_args: *const slang_SpecializationArg,
        specialization_arg_count: SlangInt,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut SlangReflectionType;
    pub fn getTypeLayout(
        &self,
        ty: *mut SlangReflectionType,
        target_index: SlangInt,
        rules: SlangLayoutRules,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut slang_TypeLayoutReflection;
    pub fn getContainerType(
        &self,
        element_type: *mut SlangReflectionType,
        container_type: slang_ContainerType,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut SlangReflectionType;
    pub fn getDynamicType(&self) -> *mut SlangReflectionType;

    fn stub_getTypeRTTIMangledName(&self);
    fn stub_getTypeConformanceWitnessMangledName(&self);
    fn stub_getTypeConformanceWitnessSequentialID(&self);
    fn stub_createCompileRequest(&self);

    pub fn createTypeConformanceComponentType(
        &self,
        ty: *mut SlangReflectionType,
        interface_ty: *mut SlangReflectionType,
        out_conformance: OutRef<ITypeConformance>,
        conformance_id_override: SlangInt,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;

    /// Returns IModule.
    pub fn loadModuleFromIRBlob(
        &self,
        module_name: *const c_char,
        path: *const c_char,
        source: Ref<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;
    pub fn getLoadedModuleCount(&self) -> SlangInt;
    pub fn getLoadedModule(&self, index: SlangInt) -> *mut c_void;

    fn stub_isBinaryModuleUpToDate(&self);

    /// Returns IModule.
    pub fn loadModuleFromSourceString(
        &self,
        module_name: *const c_char,
        path: *const c_char,
        str: *const c_char,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;

    fn stub_getDynamicObjectRTTIBytes(&self);
    fn stub_loadModuleInfoFromIRBlob(&self);

    pub fn getDeclSourceLocation(
        &self,
        decl: *mut slang_DeclReflection,
        out_location: *mut slang_SourceLocation,
    ) -> Result<()>;
}

#[interface("0c720e64-8722-4d31-8990-638a98b1c279")]
pub unsafe trait IModule: IComponentType {
    pub fn findEntryPointByName(
        &self,
        name: *const c_char,
        out_entry_point: OutRef<IEntryPoint>,
    ) -> Result<()>;
    pub fn getDefinedEntryPointCount(&self) -> SlangInt32;
    pub fn getDefinedEntryPoint(
        &self,
        index: SlangInt32,
        out_entry_point: OutRef<IEntryPoint>,
    ) -> Result<()>;
    pub fn serialize(&self, out_serialized_blob: OutRef<ISlangBlob>) -> Result<()>;
    pub fn writeToFile(&self, filename: *const c_char) -> Result<()>;
    pub fn getName(&self) -> *const c_char;
    pub fn getFilePath(&self) -> *const c_char;
    pub fn getUniqueIdentity(&self) -> *const c_char;
    pub fn findAndCheckEntryPoint(
        &self,
        name: *const c_char,
        stage: SlangStage,
        out_entry_point: OutRef<IEntryPoint>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;
    pub fn getDependencyFileCount(&self) -> SlangInt32;
    pub fn getDependencyFilePath(&self, index: SlangInt32) -> *const c_char;
    pub fn getModuleReflection(&self) -> *mut slang_DeclReflection;
    pub fn disassemble(&self, out_disassembled_blob: OutRef<ISlangBlob>) -> Result<()>;
}

#[interface("73eb3147-e544-41b5-b8f0-a244df21940b")]
pub unsafe trait ITypeConformance: IComponentType {}

#[interface("8f241361-f5bd-4ca0-a3ac-02f7fa2402b8")]
pub unsafe trait IEntryPoint: IComponentType {
    pub fn getFunctionReflection(&self) -> *mut slang_FunctionReflection;
}

#[interface("5bc42be8-5c50-4929-9e5e-d15e7c24015f")]
pub unsafe trait IComponentType: IUnknown {
    /// Returns ISession.
    pub fn getSession(&self) -> *mut c_void;
    pub fn getLayout(
        &self,
        target_index: SlangInt,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut SlangProgramLayout;
    pub fn getSpecializationParamCount(&self) -> SlangInt;
    pub fn getEntryPointCode(
        &self,
        entry_point_index: SlangInt,
        target_index: SlangInt,
        out_code: OutRef<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;

    fn stub_getResultAsFileSystem(&self);

    pub fn getEntryPointHash(
        &self,
        entry_point_index: SlangInt,
        target_index: SlangInt,
        out_hash: OutRef<ISlangBlob>,
    );
    pub fn specialize(
        &self,
        specialization_args: *const slang_SpecializationArg,
        specialization_arg_count: SlangInt,
        out_specialized_component_type: OutRef<IComponentType>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;
    pub fn link(
        &self,
        out_linked_component_type: OutRef<IComponentType>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;

    fn stub_getEntryPointHostCallable(&self);

    pub fn renameEntryPoint(
        &self,
        new_name: *const c_char,
        out_entry_point: OutRef<IComponentType>,
    ) -> Result<()>;
    pub fn linkWithOptions(
        &self,
        out_linked_component_type: OutRef<IComponentType>,
        compiler_option_entry_count: u32,
        compiler_option_entries: *const slang_CompilerOptionEntry,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;
    pub fn getTargetCode(
        &self,
        target_index: SlangInt,
        out_code: OutRef<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;
    pub fn getTargetMetadata(
        &self,
        target_index: SlangInt,
        out_metadata: OutRef<IMetadata>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;
    pub fn getEntryPointMetadata(
        &self,
        entry_point_index: SlangInt,
        target_index: SlangInt,
        out_metadata: OutRef<IMetadata>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> Result<()>;
}

#[interface("003a09fc-3a4d-4ba0-ad60-1fd863a915ab")]
pub unsafe trait ISlangFileSystem: ISlangCastable {
    pub fn loadFile(&self, path: *const c_char, out_blob: OutRef<ISlangBlob>) -> Result<()>;
}

#[interface("6264ab2b-a3e8-4a06-97f1-49bc2d2ab14d")]
pub unsafe trait ISlangSharedLibraryLoader: IUnknown {
    pub fn loadSharedLibrary(
        &self,
        path: *const c_char,
        shared_library_out: OutRef<ISlangSharedLibrary>,
    ) -> Result<()>;
}

#[interface("70dbc7c4-dc3b-4a07-ae7e-752af6a81555")]
pub unsafe trait ISlangSharedLibrary: ISlangCastable {
    pub fn findSymbolAddressByName(&self, name: *const c_char) -> *mut c_void;
}

#[interface("8044a8a3-ddc0-4b7f-af8e-026e905d7332")]
pub unsafe trait IMetadata: ISlangCastable {
    pub fn isParameterLocationUsed(
        &self,
        category: SlangParameterCategory,
        space_index: SlangUInt,
        register_index: SlangUInt,
        out_used: *mut bool,
    );
    pub fn getDebugBuildIdentifier(&self) -> *const c_char;
}

#[interface("87ede0e1-4852-44b0-8bf2-cb31874de239")]
pub unsafe trait ISlangCastable: IUnknown {
    pub fn castAs(&self, guid: &SlangUUID) -> *mut c_void;
}

#[interface("8ba5fb08-5195-40e2-ac58-0d989c3a0102")]
pub unsafe trait ISlangBlob: IUnknown {
    pub fn getBufferPointer(&self) -> *const c_void;
    pub fn getBufferSize(&self) -> usize;
}
