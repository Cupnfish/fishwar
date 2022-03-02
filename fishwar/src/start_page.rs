use std::time::Duration;

use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    math::{Rect, Size, Vec3, Vec4},
    pbr::{Material, MaterialMeshBundle, MaterialPipeline, MaterialPlugin},
    prelude::{
        shape, App, AssetServer, Assets, BuildChildren, Button, ButtonBundle, Changed, Color,
        Commands, Component, Entity, Handle, Mesh, PerspectiveCameraBundle, Plugin, Query, Res,
        ResMut, Shader, State, SystemSet, TextBundle, Transform, UiCameraBundle, With,
    },
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssetPlugin},
        render_resource::{
            std140::{AsStd140, Std140},
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
            BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer,
            BufferBindingType, BufferInitDescriptor, BufferSize, BufferUsages, ShaderStages,
        },
        renderer::RenderDevice,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, Interaction, JustifyContent, Style, UiColor, Val},
};
use bevy_tweening::{
    component_animator_system, lens::TransformScaleLens, Animator, EaseFunction, EaseMethod, Lens,
    Tween, TweeningType,
};

use crate::game_state::FishWarState;
use crate::utils::despawn_screen;
pub struct StartPagePlugin;

impl Plugin for StartPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<WavesMaterial>::default())
            .add_plugin(RenderAssetPlugin::<WavesMaterial>::default());

        app.add_system_set(SystemSet::on_enter(FishWarState::Menu).with_system(setup))
            .add_system_set(
                SystemSet::on_update(FishWarState::Menu)
                    .with_system(button_system)
                    .with_system(component_animator_system::<UiColor>),
            )
            .add_system_set(
                SystemSet::on_exit(FishWarState::Menu).with_system(despawn_screen::<StartMenu>),
            );
    }
}

const NORMAL_BUTTON: Color = Color::SEA_GREEN;
const HOVERED_BUTTON: Color = Color::DARK_GREEN;
const PRESSED_BUTTON: Color = Color::MIDNIGHT_BLUE;

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            Option<&mut Animator<Transform>>,
            Option<&mut Animator<UiColor>>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<State<FishWarState>>,
) {
    for (button, interaction, transform_tween, color_tween) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if let Some(mut tween) = transform_tween {
                    let progress = tween.tweenable().map(|tweenable| tweenable.progress());
                    if let Some(progress) = progress {
                        let target_progress = 1.0 - progress;
                        let new_tween = Tween::new(
                            EaseFunction::BackInOut,
                            TweeningType::Once,
                            Duration::from_secs_f32(0.3 + 0.3 * target_progress),
                            TransformScaleLens {
                                start: Vec3::new(0.8 + progress * 0.2, 0.8 + progress * 0.2, 0.),
                                end: Vec3::new(0.5, 0.5, 0.),
                            },
                        );
                        tween.set_tweenable(new_tween);

                        tween.set_progress(target_progress)
                    } else {
                        let new_tween = Tween::new(
                            EaseFunction::BackInOut,
                            TweeningType::Once,
                            Duration::from_secs_f32(0.3),
                            TransformScaleLens {
                                start: Vec3::new(0.8, 0.8, 0.),
                                end: Vec3::new(0.5, 0.5, 0.),
                            },
                        );
                        tween.set_tweenable(new_tween);
                    }
                }

                if let Some(mut tween) = color_tween {
                    let progress = tween.tweenable().map(|tweenable| tweenable.progress());

                    if let Some(progress) = progress {
                        let target_progress = 1.0 - progress;

                        let new_tween = Tween::new(
                            EaseMethod::Linear,
                            TweeningType::Once,
                            Duration::from_secs_f32(0.3 + 0.3 * target_progress),
                            UiColorColorLens {
                                start: {
                                    let start: Vec4 = NORMAL_BUTTON.into();
                                    let end: Vec4 = HOVERED_BUTTON.into();
                                    start.lerp(end, progress).into()
                                },
                                end: PRESSED_BUTTON,
                            },
                        );
                        tween.set_tweenable(new_tween);

                        tween.set_progress(target_progress)
                    } else {
                        let new_tween = Tween::new(
                            EaseMethod::Linear,
                            TweeningType::Once,
                            Duration::from_secs_f32(0.3),
                            UiColorColorLens {
                                start: HOVERED_BUTTON,
                                end: PRESSED_BUTTON,
                            },
                        );
                        tween.set_tweenable(new_tween);
                    }
                }
            }
            Interaction::Hovered => {
                let tween = Tween::new(
                    EaseFunction::BackInOut,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.4),
                    TransformScaleLens {
                        start: Vec3::new(1., 1., 0.),
                        end: Vec3::new(0.8, 0.8, 0.),
                    },
                );
                let color = Tween::new(
                    EaseMethod::Linear,
                    TweeningType::Once,
                    Duration::from_secs_f32(0.4),
                    UiColorColorLens {
                        start: NORMAL_BUTTON,
                        end: HOVERED_BUTTON,
                    },
                );
                commands
                    .entity(button)
                    .insert(Animator::new(tween))
                    .insert(Animator::new(color));
            }
            Interaction::None => {
                if let Some(mut tween) = transform_tween {
                    let progress = tween.tweenable().map(|tweenable| tweenable.progress());
                    let new_tween = Tween::new(
                        EaseFunction::BackInOut,
                        TweeningType::Once,
                        Duration::from_secs_f32(0.4),
                        TransformScaleLens {
                            start: Vec3::new(0.8, 0.8, 0.),
                            end: Vec3::new(1., 1., 0.),
                        },
                    );
                    tween.set_tweenable(new_tween);
                    if let Some(progress) = progress {
                        tween.set_progress(1.0 - progress)
                    }
                }
                if let Some(mut tween) = color_tween {
                    let progress = tween.tweenable().map(|tweenable| tweenable.progress());
                    let new_tween = Tween::new(
                        EaseMethod::Linear,
                        TweeningType::Once,
                        Duration::from_secs_f32(0.4),
                        UiColorColorLens {
                            start: HOVERED_BUTTON,
                            end: NORMAL_BUTTON,
                        },
                    );
                    tween.set_tweenable(new_tween);
                    if let Some(progress) = progress {
                        tween.set_progress(1.0 - progress)
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct UiColorColorLens {
    /// Start color.
    pub start: Color,
    /// End color.
    pub end: Color,
}

impl Lens<UiColor> for UiColorColorLens {
    fn lerp(&mut self, target: &mut UiColor, ratio: f32) {
        let start: Vec4 = self.start.into();
        let end: Vec4 = self.end.into();

        let value: Vec4 = start + (end - start) * ratio;

        target.0 = value.into();
    }
}
#[derive(Component)]
struct StartMenu;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WavesMaterial>>,
) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(StartMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Start",
                    TextStyle {
                        font: asset_server.load("fonts/rock-salt-regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    commands.spawn().insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(WavesMaterial::default()),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

#[derive(Debug, Copy, Clone, TypeUuid, AsStd140, Component)]
#[repr(C)]
#[uuid = "817a079c-3acf-484a-b4b3-a6254c114200"]
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

impl Default for WavesMaterial {
    fn default() -> Self {
        Self {
            width: 100.0,
            height: 100.0,
            time: 20.0,
            amplitude: 0.05,
            angular_velocity: 10.0,
            frequency: 10.0,
            offset: 0.5,
        }
    }
}

#[derive(Clone)]
pub struct GpuWavesMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for WavesMaterial {
    type ExtractedAsset = WavesMaterial;

    type PreparedAsset = GpuWavesMaterial;

    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let contents = extracted_asset.as_std140();
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: contents.as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuWavesMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}
impl Material for WavesMaterial {
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
                    min_binding_size: BufferSize::new(WavesMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/fragment.spv"))
    }
}
