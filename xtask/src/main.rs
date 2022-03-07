mod flags;

use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use anyhow::Result;
use xshell::{cmd, pushd};

fn main() -> Result<()> {
    let _d = pushd(project_root())?;

    let flags = flags::Xtask::from_env()?;

    match flags.subcommand {
        flags::XtaskCmd::Help(_) => {
            println!("{}", flags::Xtask::HELP);
            Ok(())
        }
        flags::XtaskCmd::Shader(cmd) => {
            cmd!("cargo +nightly-2022-01-13 run -r -p shaders-builder")
                .run()
                .expect("Building sharders failed.");

            if cmd.gen_all {
                let dir = std::fs::read_dir("inject/assets/shaders")?;
                for file in dir
                    .filter_map(|entry| entry.ok())
                    .filter(|f| f.path().extension().eq(&Some(&OsStr::new("spv"))))
                {
                    let src_path = file.path();
                    let dest_name = Path::new(src_path.file_stem().unwrap()).with_extension("wgsl");
                    let dest_path = src_path.parent().unwrap().join(dest_name);
                    if let Err(e) = cmd!("naga {src_path} {dest_path}").run() {
                        eprintln!("Failed to generate wgsl,source: {src_path:?}.\n Error: {e:?}")
                    };
                }
            }

            Ok(())
        }
    }

    // cmd!("cargo +nightly-2022-01-13 run -r -p shaders-builder")
    //     .run()
    //     .expect("Building sharders failed.");

    // Ok(())
}

fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}
