use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder};
use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let builder_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    let extensions = &[];

    let capabilities = &[];

    compile_shader_multi(&builder_root, extensions, capabilities)?;

    Ok(())
}

fn compile_shader_multi(
    root_path: &Path,
    extensions: &[&str],
    capabilities: &[Capability],
) -> Result<(), Box<dyn Error>> {
    let mut builder = SpirvBuilder::new(
        root_path.join("../shaders/"),
        "spirv-unknown-vulkan1.1spv1.4",
    )
    .print_metadata(MetadataPrintout::DependencyOnly)
    .multimodule(true);

    for extension in extensions {
        builder = builder.extension(*extension);
    }

    for capability in capabilities {
        builder = builder.capability(*capability);
    }

    let result = builder.build()?;

    for (name, path) in result.module.unwrap_multi() {
        let name = handle_name(name);
        std::fs::copy(
            path,
            root_path.join(&format!("../inject/assets/shaders/{}.spv", name)),
        )?;
    }

    Ok(())
}

fn handle_name(name: &str) -> &str {
    name
}
