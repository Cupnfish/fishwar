#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use spirv_std::glam::{vec2, vec4, Vec4, Vec4Swizzles};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[repr(C)]
pub struct WavesMaterial {
    width: f32,
    height: f32,
    time: f32,
    // 振幅（控制波浪顶端和底端的高度）
    amplitude: f32,
    // 角速度（控制波浪的周期）
    angular_velocity: f32,
    // 频率（控制波浪移动的速度）
    frequency: f32,
    // 偏距（设为 0.5 使得波浪垂直居中于屏幕）
    offset: f32,
}

#[spirv(fragment(entry_point_name = "fragment"))]
pub fn waves_frag(
    #[spirv(frag_coord)] in_frag_coord: Vec4,
    #[spirv(uniform, descriptor_set = 2, binding = 0)] waves_material: &WavesMaterial,
    output: &mut Vec4,
) {
    // 将像素坐标归一化（区间 [0.0, 1.0]）
    let uv = in_frag_coord.xy() / vec2(waves_material.width, waves_material.height);

    // 初相位（正值表现为向左移动，负值则表现为向右移动）
    // iTime 是 Shadertoy 提供的运行时间全局变量（类型：float）
    let initial_phase = waves_material.frequency * waves_material.time;

    // 代入正弦曲线公式计算 y 值
    // y = Asin(ωx ± φt) + k
    let y = waves_material.amplitude
        * (waves_material.angular_velocity * uv.x + initial_phase).sin()
        + waves_material.offset;

    // 区分 y 值上下部分，设置不同颜色
    let color = if uv.y > y {
        vec4(0.0, 0.0, 0.0, 1.0)
    } else {
        vec4(0.0, 0.7, 0.9, 1.0)
    };

    // 输出到屏幕
    *output = color;
}
