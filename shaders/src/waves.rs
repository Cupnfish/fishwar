// #[cfg(not(target_arch = "spirv"))]
// use bevy_crevice::std140::AsStd140;
// #[cfg(not(target_arch = "spirv"))]
// use bevy_ecs::component::Component;
// #[cfg(not(target_arch = "spirv"))]
// use bevy_reflect::Reflect;
#[cfg(not(target_arch = "spirv"))]
use spirv_std::macros::spirv;

use spirv_std::glam::{Vec2, Vec3, Vec4, Vec4Swizzles};

#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

#[derive(Copy, Clone)]
#[cfg_attr(not(target_arch = "spirv"), derive(Debug, PartialEq))]
#[repr(C)]
pub struct Properties {
    /// 振幅（控制波浪顶端和底端的高度）
    ///
    /// 曲线最高点与最低点的差值，表现为曲线的整体高度
    pub amplitude: f32,
    /// 角速度（控制波浪的周期）
    ///
    /// 控制曲线的周期，表现为曲线的紧密程度
    pub angular_velocity: f32,
    /// 频率（控制波浪移动的速度）
    pub frequency: f32,
    /// 偏距（设为 0.5 使得波浪垂直居中于屏幕）
    pub offset: f32,
    // 底色
    pub color: Vec4,
    // 时间
    pub time: f32,
}

#[spirv(fragment(entry_point_name = "fragment"))]
pub fn waves_frag(
    _world_position: Vec4,
    _world_normal: Vec3,
    uv: Vec2,
    #[spirv(frag_coord)] in_frag_coord: Vec4,
    #[spirv(uniform, descriptor_set = 1, binding = 0)] properties: &Properties,
    output: &mut Vec4,
) {
    // 直接丢弃原本就透明的像素
    if properties.color.w == 0.0 {
        return;
    }

    // 初相位（正值表现为向左移动，负值则表现为向右移动）
    // cc_time 是 Cocos Creator 提供的运行时间全局变量（类型：vec4）
    let initia_phase = properties.frequency * properties.time;

    // 代入正弦曲线公式计算 y 值
    // y = Asin(ωx ± φt) + k
    let y = properties.amplitude * (properties.angular_velocity + uv.x + initia_phase).sin()
        + properties.offset;

    if uv.y < y {
        return;
    }

    // 输出颜色
    *output = properties
        .color
        .lerp(in_frag_coord.xyz().normalize().extend(1.0), 0.2);
}
