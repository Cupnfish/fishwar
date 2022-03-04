use bevy::{
    core::Time,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::Vec4,
    prelude::{
        warn, App, AssetServer, Assets, Color, Component, Handle, Plugin, Query, Res, ResMut,
        Shader, State,
    },
    reflect::{Reflect, TypeUuid},
    render::{
        render_asset::RenderAsset,
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType,
            BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin},
};
use bevy_tweening::{asset_animator_system, Lens, Lerp};

use crate::game_state::FishWarState;

#[derive(Debug, Copy, Clone, TypeUuid, Component, Reflect, AsStd140, PartialEq)]
#[uuid = "817a079c-3acf-484a-b4b3-a6254c114200"]
#[cfg_attr(feature = "dev", derive(bevy_inspector_egui::Inspectable))]
pub struct WavesMaterial {
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

impl Default for WavesMaterial {
    fn default() -> Self {
        Self {
            amplitude: 3.0,
            angular_velocity: 0.3,
            frequency: 5.,
            offset: 1.0,
            color: Color::GREEN.into(),
            time: Default::default(),
        }
    }
}

#[derive(Clone, Component)]
pub struct GpuWavesMaterial {
    // properties_buffer: Buffer,
    // time_buffer: Buffer,
    bind_group: BindGroup,
}

pub struct WavesPlugin;

impl Plugin for WavesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Offset>()
            .add_plugin(Material2dPlugin::<WavesMaterial>::default())
            .add_system(asset_animator_system::<WavesMaterial>);
        #[cfg(feature = "dev")]
        {
            let mut registry = app
                .world
                .get_resource_or_insert_with(bevy_inspector_egui::InspectableRegistry::default);

            registry.register::<WavesMaterial>();
        }
    }
}

pub fn sync_with_time(
    mut materials: ResMut<Assets<WavesMaterial>>,
    query_waves: Query<&Handle<WavesMaterial>>,
    time: Res<Time>,
    offset: Res<Offset>,
    mut game_state: ResMut<State<FishWarState>>,
) {
    for handle in query_waves.iter() {
        if let Some(waves) = materials.get_mut(handle) {
            waves.time = time.seconds_since_startup() as f32;
            if waves.offset <= 0.0 {
                if let Err(e) = game_state.set(FishWarState::Game) {
                    warn!("set state error: {:?}", e);
                };
            }

            if waves.offset <= 1.0 {
                waves.offset += offset.0;
            } else {
                waves.offset = 1.0;
            }
        }
    }
}

#[derive(Clone)]
pub struct Offset(pub f32);

impl Default for Offset {
    fn default() -> Self {
        Self(NORMAL_OFFSET)
    }
}

const NORMAL_OFFSET: f32 = 0.005;

impl Offset {
    pub fn off(&mut self) {
        if self.0.is_sign_negative() {
            self.0 = NORMAL_OFFSET;
        }
    }
    pub fn on(&mut self) {
        if self.0.is_sign_positive() {
            self.0 = -NORMAL_OFFSET * 10.0;
        }
    }
    pub fn hoverd_on(&mut self) {
        if self.0.is_sign_positive() {
            self.0 = -NORMAL_OFFSET * 3.0;
        }
    }
}

impl RenderAsset for WavesMaterial {
    type ExtractedAsset = Self;

    type PreparedAsset = GpuWavesMaterial;

    type Param = (SRes<RenderDevice>, SRes<Material2dPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<
        Self::PreparedAsset,
        bevy::render::render_asset::PrepareAssetError<Self::ExtractedAsset>,
    > {
        let properties_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: extracted_asset.as_std140().as_bytes(),
            label: Some("properties_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: properties_buffer.as_entire_binding(),
            }],
            label: None,
            layout: &pipeline.material2d_layout,
        });
        Ok(GpuWavesMaterial {
            // properties_buffer,
            // time_buffer,
            bind_group,
        })
    }
}

impl Material2d for WavesMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/fragment.spv"))
    }

    fn bind_group(material: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Self::std140_size_static() as u64),
                },
                count: None,
            }],
            label: Some("waves bind group layout"),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WavesPropertiesLens {
    pub start: WavesMaterial,
    pub end: WavesMaterial,
}

impl Lens<WavesMaterial> for WavesPropertiesLens {
    fn lerp(&mut self, target: &mut WavesMaterial, ratio: f32) {
        let Self { start, end } = self;
        let color = start.color.lerp(end.color, ratio);
        let frequency = start.frequency.lerp(&end.frequency, &ratio);
        let amplitude = start.amplitude.lerp(&end.amplitude, &ratio);
        let angular_velocity = start.angular_velocity.lerp(&end.angular_velocity, &ratio);

        target.color = color;
        target.frequency = frequency;
        target.amplitude = amplitude;
        target.angular_velocity = angular_velocity;
    }
}
