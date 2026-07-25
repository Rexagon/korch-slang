# korch-slang

[Slang](https://shader-slang.org/) bindings for the Korch game engine.

## Basic usage

```rust
use korch_slang::*;

// At first we need to load an Slang compiler library.
let ctx = SlangContext::new("/usr/local/lib/libslang-compiler.so")?;
// Then we can set a writer for a diagnostic output.
// By default it's `std::io::sink()` and nothing is visible.
ctx.set_diagnostics_writer(std::io::stdout());

// Create a `GlobalSession` and provide paths for required compilers.
let global_session = ctx.create_global_session(
    CompilerPaths::new()
        .dxil("/usr/local/lib/libdxil.so")
        .dxcompiler("/usr/local/lib/libdxcompiler.so")
)?;

// Get target profile ID.
let profile = global_session.find_profile("sm_6_5")?;

// Create compilation session. Each unique set of compiler options
// requires a new session. Use specialization to keep them low.
let session = global_session.create_session(&SessionDescriptor {
    // Directories where to search for shaders by path.
    search_paths: &[
        "assets/lib",
        "assets/materials",
        "assets/builtin",
    ],
    targets: &[
        TargetDescriptor {
            format: CompileTarget::DXIL,
            profile,
            options: CompilerOptions::default(),
        },
    ],
    preprocessor_macros: &[("ENGINE_NAME", "myengine3000".into())],
    options: CompilerOptions::default().diagnostic_color(DiagnosticColor::Always),
    ..Default::default()
})?;

// Load your shader.
let module = session.load_module(
    "test",
    Some(Path::new("test.slang")),
    include_str!("test.slang"),
)?;

// And compiler each entry point in it.
for entry_point in module.entry_points_iter() {
    println!("entry_point: {}", entry_point.get_name()?);
    println!("stage: {:?}", entry_point.get_stage()?);

    let linked = session
        .combine_component_types([
            module.as_boxed(),
            entry_point.as_boxed(),
        ])?
        .link()?;

    let compiled = linked.get_entry_point_code(0, 0)?;
    // ... save compiled into file or other stuff
}
```

See more examples [here](./examples).

> NOTE: This library supports both Linux and Windows. Simply specify the paths to the `so` or `dll` compiler libraries, depending on the target platform.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
