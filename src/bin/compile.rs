use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use korch_slang::com::ISlangSharedLibraryLoader;
use korch_slang::{
    CompileTarget, CustomSharedLibraryLoader, GlobalSession, LoadedLibrary, PassThrough,
};

fn main() -> Result<()> {
    #[cfg(feature = "env_logger")]
    env_logger::init();

    unsafe {
        compile_shader(
            "/home/ivan/Downloads/slang-2026.12.0.1-windows-x86_64/bin/slang-compiler.dll",
        )?
    };

    println!("done");
    Ok(())
}

unsafe fn compile_shader(path: &str) -> Result<()> {
    let lib =
        unsafe { libloading::Library::new(path).context("failed to open `slang-compiler.dll`")? };

    let loaded = LoadedLibrary::new(&lib)?;

    let global_session = GlobalSession::new(&loaded)?;
    println!("slang version: {}", global_session.get_build_tag());

    let custom_loader: ISlangSharedLibraryLoader = CustomSharedLibraryLoader {
        base_dir: PathBuf::from(path)
            .parent()
            .unwrap_or(Path::new("./"))
            .to_path_buf(),
    }
    .into();

    global_session.set_library_loader(Some(&custom_loader));

    global_session.check_compile_target_support(CompileTarget::DXIL)?;
    global_session.check_pass_through_support(PassThrough::DXC)?;

    Ok(())
}
