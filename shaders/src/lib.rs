#![cfg_attr(
    target_arch = "spirv",
    no_std,
    feature(register_attr, asm_experimental_arch),
    register_attr(spirv)
)]
#![allow(clippy::too_many_arguments)]

pub mod waves;
