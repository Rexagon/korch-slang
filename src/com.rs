#![allow(non_snake_case)]

use std::ffi::{c_char, c_void};

use windows_core::{HRESULT, IUnknown, IUnknown_Vtbl, OutRef, Ref, interface};

use crate::sys::{
    SlangBool, SlangCompileRequest, SlangCompileTarget, SlangInt, SlangInt32, SlangLayoutRules,
    SlangParameterCategory, SlangPassThrough, SlangPassThroughIntegral, SlangProfileID,
    SlangProgramLayout, SlangStage, SlangUInt, SlangUUID, SlangWriterMode,
    slang_CompilerOptionEntry, slang_ContainerType, slang_DeclReflection, slang_FunctionReflection,
    slang_SessionDesc, slang_SourceLocation, slang_SpecializationArg, slang_TypeLayoutReflection,
    slang_TypeReflection,
};

#[interface("c140b5fd-0c78-452e-ba7c-1a1e70c7f71c")]
pub unsafe trait IGlobalSession: IUnknown {
    pub fn createSession(
        &mut self,
        desc: *const slang_SessionDesc,
        out_session: OutRef<ISession>,
    ) -> HRESULT;
    pub fn findProfile(&mut self, name: *const c_char) -> SlangProfileID;

    fn stub_setDownstreamCompilerPath(&mut self);
    fn stub_setDownstreamCompilerPrelude(&mut self);
    fn stub_getDownstreamCompilerPrelude(&mut self);

    pub fn getBuildTagString(&mut self) -> *const c_char;

    fn stub_setDefaultDownstreamCompiler(&mut self);
    fn stub_getDefaultDownstreamCompiler(&mut self);
    fn stub_setLanguagePrelude(&mut self);
    fn stub_getLanguagePrelude(&mut self);

    pub fn createCompileRequest(
        &mut self,
        out_compile_request: *mut *mut SlangCompileRequest,
    ) -> HRESULT;

    fn stub_addBuiltins(&mut self);

    pub fn setSharedLibraryLoader(&mut self, loader: Ref<ISlangSharedLibraryLoader>);
    pub fn getSharedLibraryLoader(&mut self) -> *mut c_void;

    pub fn checkCompileTargetSupport(&mut self, target: SlangCompileTarget) -> HRESULT;
    pub fn checkPassThroughSupport(&mut self, pass_through: SlangPassThrough) -> HRESULT;

    fn stub_compileCoreModule(&mut self);
    fn stub_loadCoreModule(&mut self);
    fn stub_saveCoreModule(&mut self);

    fn stub_findCapability(&mut self);
    fn stub_setDownstreamCompilerForTransition(&mut self);
    fn stub_getDownstreamCompilerForTransition(&mut self);
    fn stub_getCompilerElapsedTime(&mut self);
    fn stub_setSPIRVCoreGrammar(&mut self);
    fn stub_parseCommandLineArguments(&mut self);
    fn stub_getSessionDescDigest(&mut self);
    fn stub_compileBuiltinModule(&mut self);
    fn stub_loadBuiltinModule(&mut self);
    fn stub_saveBuiltinModule(&mut self);

    // NOTE: Can't call this for some reason.
    pub fn getDownstreamCompilerVersion(
        &mut self,
        pass_through: SlangPassThroughIntegral,
        out_major: *mut i32,
        out_minor: *mut i32,
    ) -> HRESULT;
}

#[interface("67618701-d116-468f-ab3b-474bedce0e3d")]
pub unsafe trait ISession: IUnknown {
    /// Returns IGlobalSession.
    pub fn getGlobalSession(&mut self) -> *mut c_void;
    /// Returns IModule.
    pub fn loadModule(
        &mut self,
        module_name: *const c_char,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;
    /// Returns IModule.
    pub fn loadModuleFromSource(
        &mut self,
        module_name: *const c_char,
        path: *const c_char,
        source: Ref<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;
    pub fn createCompositeComponentType(
        &mut self,
        component_types: *const IComponentType,
        component_type_count: SlangInt,
        out_composite_component_type: OutRef<IComponentType>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;
    pub fn specializeType(
        &mut self,
        type_reflection: *mut slang_TypeReflection,
        specialization_args: *const slang_SpecializationArg,
        specialization_arg_count: SlangInt,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut slang_TypeReflection;
    pub fn getTypeLayout(
        &mut self,
        ty: *mut slang_TypeReflection,
        target_index: SlangInt,
        rules: SlangLayoutRules,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut slang_TypeLayoutReflection;
    pub fn getContainerType(
        &mut self,
        element_type: *mut slang_TypeReflection,
        container_type: slang_ContainerType,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut slang_TypeReflection;
    pub fn getDynamicType(&mut self) -> *mut slang_TypeReflection;

    fn stub_getTypeRTTIMangledName(&mut self);
    fn stub_getTypeConformanceWitnessMangledName(&mut self);
    fn stub_getTypeConformanceWitnessSequentialID(&mut self);
    fn stub_createCompileRequest(&mut self);

    pub fn createTypeConformanceComponentType(
        &mut self,
        ty: *mut slang_TypeReflection,
        interface_ty: *mut slang_TypeReflection,
        out_conformance: OutRef<ITypeConformance>,
        conformance_id_override: SlangInt,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;

    /// Returns IModule.
    pub fn loadModuleFromIRBlob(
        &mut self,
        module_name: *const c_char,
        path: *const c_char,
        source: Ref<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;
    pub fn getLoadedModuleCount(&mut self) -> SlangInt;
    pub fn getLoadedModule(&mut self, index: SlangInt) -> *mut c_void;

    fn stub_isBinaryModuleUpToDate(&mut self);

    /// Returns IModule.
    pub fn loadModuleFromSourceString(
        &mut self,
        module_name: *const c_char,
        path: *const c_char,
        str: *const c_char,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut c_void;

    fn stub_getDynamicObjectRTTIBytes(&mut self);
    fn stub_loadModuleInfoFromIRBlob(&mut self);

    pub fn getDeclSourceLocation(
        &mut self,
        decl: *mut slang_DeclReflection,
        out_location: *mut slang_SourceLocation,
    ) -> HRESULT;
}

#[interface("0c720e64-8722-4d31-8990-638a98b1c279")]
pub unsafe trait IModule: IComponentType {
    pub fn findEntryPointByName(
        &mut self,
        name: *const c_char,
        out_entry_point: OutRef<IEntryPoint>,
    ) -> HRESULT;
    pub fn getDefinedEntryPointCount(&mut self) -> SlangInt32;
    pub fn getDefinedEntryPoint(
        &mut self,
        index: SlangInt32,
        out_entry_point: OutRef<IEntryPoint>,
    ) -> HRESULT;
    pub fn serialize(&mut self, out_serialized_blob: OutRef<ISlangBlob>) -> HRESULT;
    pub fn writeToFile(&mut self, filename: *const c_char) -> HRESULT;
    pub fn getName(&mut self) -> *const c_char;
    pub fn getFilePath(&mut self) -> *const c_char;
    pub fn getUniqueIdentity(&mut self) -> *const c_char;
    pub fn findAndCheckEntryPoint(
        &mut self,
        name: *const c_char,
        stage: SlangStage,
        out_entry_point: OutRef<IEntryPoint>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;
    pub fn getDependencyFileCount(&mut self) -> SlangInt32;
    pub fn getDependencyFilePath(&mut self, index: SlangInt32) -> *const c_char;
    pub fn getModuleReflection(&mut self) -> *mut slang_DeclReflection;
    pub fn disassemble(&mut self, out_disassembled_blob: OutRef<ISlangBlob>) -> HRESULT;
}

#[interface("73eb3147-e544-41b5-b8f0-a244df21940b")]
pub unsafe trait ITypeConformance: IComponentType {}

#[interface("8f241361-f5bd-4ca0-a3ac-02f7fa2402b8")]
pub unsafe trait IEntryPoint: IComponentType {
    pub fn getFunctionReflection(&mut self) -> *mut slang_FunctionReflection;
}

#[interface("5bc42be8-5c50-4929-9e5e-d15e7c24015f")]
pub unsafe trait IComponentType: IUnknown {
    /// Returns ISession.
    pub fn getSession(&mut self) -> *mut c_void;
    pub fn getLayout(
        &mut self,
        target_index: SlangInt,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> *mut SlangProgramLayout;
    pub fn getSpecializationParamCount(&mut self) -> SlangInt;
    pub fn getEntryPointCode(
        &mut self,
        entry_point_index: SlangInt,
        target_index: SlangInt,
        out_code: OutRef<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;

    fn stub_getResultAsFileSystem(&mut self);

    pub fn getEntryPointHash(
        &mut self,
        entry_point_index: SlangInt,
        target_index: SlangInt,
        out_hash: OutRef<ISlangBlob>,
    );
    pub fn specialize(
        &mut self,
        specialization_args: *const slang_SpecializationArg,
        specialization_arg_count: SlangInt,
        out_specialized_component_type: OutRef<IComponentType>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;
    pub fn link(
        &mut self,
        out_linked_component_type: OutRef<IComponentType>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;

    fn stub_getEntryPointHostCallable(&mut self);

    pub fn renameEntryPoint(
        &mut self,
        new_name: *const c_char,
        out_entry_point: OutRef<IComponentType>,
    ) -> HRESULT;
    pub fn linkWithOptions(
        &mut self,
        out_linked_component_type: OutRef<IComponentType>,
        compiler_option_entry_count: u32,
        compiler_option_entries: *const slang_CompilerOptionEntry,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;
    pub fn getTargetCode(
        &mut self,
        target_index: SlangInt,
        out_code: OutRef<ISlangBlob>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;
    pub fn getTargetMetadata(
        &mut self,
        target_index: SlangInt,
        out_metadata: OutRef<IMetadata>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;
    pub fn getEntryPointMetadata(
        &mut self,
        entry_point_index: SlangInt,
        target_index: SlangInt,
        out_metadata: OutRef<IMetadata>,
        out_diagnostics: OutRef<ISlangBlob>,
    ) -> HRESULT;
}

#[interface("ec457f0e-9add-4e6b-851c-d7fa716d15fd")]
pub unsafe trait ISlangWriter: IUnknown {
    pub fn beginAppendBuffer(&mut self, max_num_chars: usize) -> *mut c_char;
    pub fn endAppendBuffer(&mut self, buffer: *mut c_char, num_chars: usize) -> HRESULT;
    pub fn write(&mut self, chars: *mut c_char, num_chars: usize) -> HRESULT;
    pub fn flush(&mut self);
    pub fn is_console(&mut self) -> SlangBool;
    pub fn setMode(&mut self, mode: SlangWriterMode) -> HRESULT;
}

#[interface("003a09fc-3a4d-4ba0-ad60-1fd863a915ab")]
pub unsafe trait ISlangFileSystem: ISlangCastable {
    pub fn loadFile(&mut self, path: *const c_char, out_blob: OutRef<ISlangBlob>) -> HRESULT;
}

#[interface("6264ab2b-a3e8-4a06-97f1-49bc2d2ab14d")]
pub unsafe trait ISlangSharedLibraryLoader: IUnknown {
    pub fn loadSharedLibrary(
        &self,
        path: *const c_char,
        shared_library_out: OutRef<ISlangSharedLibrary>,
    ) -> HRESULT;
}

#[interface("70dbc7c4-dc3b-4a07-ae7e-752af6a81555")]
pub unsafe trait ISlangSharedLibrary: ISlangCastable {
    pub fn findSymbolAddressByName(&self, name: *const c_char) -> *mut c_void;
}

#[interface("8044a8a3-ddc0-4b7f-af8e-026e905d7332")]
pub unsafe trait IMetadata: ISlangCastable {
    pub fn isParameterLocationUsed(
        &mut self,
        category: SlangParameterCategory,
        space_index: SlangUInt,
        register_index: SlangUInt,
        out_used: *mut bool,
    );
    pub fn getDebugBuildIdentifier(&mut self) -> *const c_char;
}

#[interface("87ede0e1-4852-44b0-8bf2-cb31874de239")]
pub unsafe trait ISlangCastable: IUnknown {
    pub fn castAs(&mut self, guid: &SlangUUID) -> *mut c_void;
}

#[interface("8ba5fb08-5195-40e2-ac58-0d989c3a0102")]
pub unsafe trait ISlangBlob: IUnknown {
    pub fn getBufferPointer(&self) -> *const c_void;
    pub fn getBufferSize(&self) -> usize;
}
