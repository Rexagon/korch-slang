use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use korch_slang::{
    AsBoxedComponentType, CompileTarget, CompilerPaths, PassThrough, SessionDescriptor,
    SlangContext, SpecializationArg, TargetDescriptor, TypeConformanceDescriptor,
};

struct Config {
    slang_path: PathBuf,
    dxil_path: PathBuf,
    dxcompiler_path: PathBuf,
}

fn main() -> Result<()> {
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
    let ctx = SlangContext::new(&config.slang_path)?;
    let global_session = ctx.create_global_session(CompilerPaths {
        dxil: Some(config.dxil_path),
        dxcompiler: Some(config.dxcompiler_path),
    })?;
    println!("slang version: {}", global_session.get_build_tag());

    global_session.check_compile_target_support(CompileTarget::DXIL)?;
    global_session.check_pass_through_support(PassThrough::DXC)?;

    let profile = global_session
        .find_profile("sm_6_5")
        .context("target profile not found")?;
    println!("profile_id: {profile:?}");

    let session = global_session
        .create_session(&SessionDescriptor {
            search_paths: &[],
            targets: &[
                TargetDescriptor {
                    format: CompileTarget::DXIL,
                    profile,
                },
                TargetDescriptor {
                    format: CompileTarget::HLSL,
                    profile,
                },
            ],
            preprocessor_macros: &[],
            ..Default::default()
        })
        .context("failed to create session")?;

    let module = session.load_module(
        "test",
        Some(Path::new("test.slang")),
        include_str!("test.slang"),
    )?;

    for entry_point in module.entry_points_iter() {
        println!("entry_point: {}", entry_point.get_name()?);
        println!("stage: {:?}", entry_point.get_stage()?);

        let layout = module.get_layout()?;
        let high_quality = layout.find_type("HighQuality").context("no type")?;
        let quality_interface = layout.find_type("IQuality").context("no type")?;

        let specialized = entry_point
            .specialize(&[SpecializationArg::Type(high_quality.clone())])
            .context("failed to specialize entry point")?;

        let conformance_high_quality =
            session.create_type_conformance(&TypeConformanceDescriptor {
                interface: quality_interface,
                ty: high_quality,
                override_id: Some(0),
            })?;

        let linked = session
            .combine_component_types([
                module.as_boxed(),
                specialized.as_boxed(),
                conformance_high_quality.as_boxed(),
            ])
            .context("failed to combine component types")?
            .link()
            .context("failed to link component types")?;

        let compiled = linked
            .get_entry_point_code(0, 0)
            .context("failed to compile entry code")?;
        println!("compiled {} bytes", compiled.len());

        let hlsl_code = linked
            .get_entry_point_code(0, 1)
            .context("failed to compile to HLSL")?;
        let hlsl_code = String::from_utf8(hlsl_code).context("invalid HLSL code")?;
        println!("----\n{hlsl_code}");
    }

    Ok(())
}
