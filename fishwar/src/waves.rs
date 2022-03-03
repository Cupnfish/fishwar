use bevy::{
    core::Time,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::{
        App, AssetServer, Assets, Color, Commands, Component, Handle, Plugin, Query, Res, ResMut,
        Shader,
    },
    reflect::TypeUuid,
    render::{
        render_asset::{RenderAsset, RenderAssets},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::{RenderDevice, RenderQueue},
        RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin},
};
use shaders::waves;

#[derive(Debug, Copy, Clone, TypeUuid, Component)]
#[repr(transparent)]
#[uuid = "817a079c-3acf-484a-b4b3-a6254c114200"]
pub struct WavesMaterial(pub waves::Properties);

impl Default for WavesMaterial {
    fn default() -> Self {
        Self(waves::Properties {
            amplitude: 0.05,
            angular_velocity: 0.1,
            frequency: 10.0,
            offset: 0.5,
            color: Color::GREEN.into(),
            time: Default::default(),
        })
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
        app.add_plugin(Material2dPlugin::<WavesMaterial>::default())
            .add_system(sync_with_time);
    }
}

fn sync_with_time(
    mut materials: ResMut<Assets<WavesMaterial>>,
    query_waves: Query<&Handle<WavesMaterial>>,
    time: Res<Time>,
) {
    for handle in query_waves.iter() {
        if let Some(waves) = materials.get_mut(handle) {
            waves.0.time = time.seconds_since_startup() as f32;
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
            contents: extracted_asset.0.as_std140().as_bytes(),
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
                    min_binding_size: BufferSize::new(
                        waves::Properties::std140_size_static() as u64
                    ),
                },
                count: None,
            }],
            label: Some("waves bind group layout"),
        })
    }
}
