use std::path::PathBuf;

use anyhow::{Context, Result};
use korch_slang::com::ISlangSharedLibraryLoader;
use korch_slang::{
    CompileTarget, CustomSharedLibraryLoader, GlobalSession, LoadedLibrary, PassThrough,
};

struct Config {
    slang_path: PathBuf,
    dxil_path: PathBuf,
    dxcompiler_path: PathBuf,
}

fn main() -> Result<()> {
    #[cfg(feature = "env_logger")]
    env_logger::init();

    compile_shader(Config {
        slang_path: "/usr/local/lib/libslang-compiler.so".into(),
        dxil_path: "/usr/local/lib/libdxil.so".into(),
        dxcompiler_path: "/usr/local/lib/libdxcompiler.so".into(),
    })?;

    println!("done");
    Ok(())
}

fn compile_shader(config: Config) -> Result<()> {
    let lib = unsafe {
        libloading::Library::new(&config.slang_path)
            .context("failed to open Slang compiler library")?
    };

    let loaded = LoadedLibrary::new(&lib)?;

    let global_session = GlobalSession::new(&loaded)?;
    println!("slang version: {}", global_session.get_build_tag());

    let custom_loader: ISlangSharedLibraryLoader = CustomSharedLibraryLoader {
        dxil_path: config.dxil_path,
        dxcompiler_path: config.dxcompiler_path,
    }
    .into();

    global_session.set_library_loader(Some(&custom_loader));

    global_session.check_compile_target_support(CompileTarget::DXIL)?;
    global_session.check_pass_through_support(PassThrough::DXC)?;

    Ok(())
}
