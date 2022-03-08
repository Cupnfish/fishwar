mod flags;

use std::{
    env,
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
        flags::XtaskCmd::Shader(_) => {
            cmd!("cargo +nightly-2022-01-13 run -r -p shaders-builder")
                .run()
                .expect("Building sharders failed.");
            Ok(())
        }
    }
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
