#![allow(non_snake_case)]

use std::ffi::{c_char, c_void};

use windows_core::{HRESULT, IUnknown, IUnknown_Vtbl, Ref, interface};

use crate::sys::root::{
    SlangCompileRequest, SlangCompileTargetIntegral, SlangPassThroughIntegral, SlangUUID,
};

#[interface("c140b5fd-0c78-452e-ba7c-1a1e70c7f71c")]
pub unsafe trait IGlobalSession: IUnknown {
    fn stub_createSession(&mut self);
    fn stub_findProfile(&mut self);
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

    pub fn checkCompileTargetSupport(&mut self, target: SlangCompileTargetIntegral) -> HRESULT;
    pub fn checkPassThroughSupport(&mut self, pass_through: SlangPassThroughIntegral) -> HRESULT;

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

#[interface("6264ab2b-a3e8-4a06-97f1-49bc2d2ab14d")]
pub unsafe trait ISlangSharedLibraryLoader: IUnknown {
    pub fn loadSharedLibrary(
        &self,
        path: *const c_char,
        shared_library_out: *mut ISlangSharedLibrary,
    ) -> HRESULT;
}

#[interface("70dbc7c4-dc3b-4a07-ae7e-752af6a81555")]
pub unsafe trait ISlangSharedLibrary: ISlangCastable {
    pub fn findSymbolAddressByName(&self, name: *const c_char) -> *mut c_void;
}

#[interface("87ede0e1-4852-44b0-8bf2-cb31874de239")]
pub unsafe trait ISlangCastable: IUnknown {
    pub fn castAs(&mut self, guid: &SlangUUID) -> *mut c_void;
}
