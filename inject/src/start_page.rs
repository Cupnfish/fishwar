use std::time::Duration;

use bevy::{
    core::Time,
    math::{Rect, Size, Vec2, Vec3, Vec4},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    text::{Text, TextStyle},
    ui::{AlignItems, Interaction, JustifyContent, Style, UiColor, Val},
    window::{WindowResized, Windows},
};
use bevy_tweening::{
    component_animator_system, lens::TransformScaleLens, Animator, AssetAnimator, EaseFunction,
    EaseMethod, Lens, Tween, TweeningType,
};

use crate::utils::despawn_screen;
use crate::{
    game_state::FishWarState,
    waves::{WavesMaterial, WavesPropertiesLens},
};
pub struct StartPagePlugin;

impl Plugin for StartPagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(FishWarState::Menu).with_system(setup))
            .add_system_set(
                SystemSet::on_update(FishWarState::Menu)
                    .with_system(button_system)
                    .with_system(component_animator_system::<UiColor>)
                    .with_system(sync_with_time)
                    .with_system(sync_with_window_size),
            )
            .add_system_set(
                SystemSet::on_exit(FishWarState::Menu)
                    .with_system(despawn_screen::<StartMenu>)
                    .with_system(remove_resource),
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
    mut offset: ResMut<Offset>,
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
                offset.on();
            }
            Interaction::Hovered => {
                offset.hoverd_on();
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
                offset.off();
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
pub struct StartMenu;

#[derive(Component)]
pub struct Wave;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WavesMaterial>>,
    windows: Res<Windows>,
) {
    commands.insert_resource(Offset::default());

    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(StartMenu);

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
    let lens = WavesPropertiesLens {
        start: WavesMaterial {
            amplitude: 0.2,
            angular_velocity: 0.8,
            frequency: 3.,
            color: Color::SEA_GREEN.into(),
            time: Default::default(),
            offset: Default::default(),
        },
        end: WavesMaterial {
            amplitude: 0.15,
            angular_velocity: 0.8,
            frequency: 3.5,
            color: Color::PINK.into(),
            time: Default::default(),
            offset: Default::default(),
        },
    };
    let tween = Tween::new(
        EaseMethod::Linear,
        TweeningType::PingPong,
        Duration::from_secs_f32(15.0),
        lens,
    );

    let waves = materials.add(WavesMaterial::default());
    let window = windows.get_primary().unwrap();

    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes
                .add(new_waves_mesh(window.width(), window.height()))
                .into(),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                rotation: Default::default(),
                scale: Vec3::splat(1.),
            },
            material: waves.clone(),
            ..Default::default()
        })
        .insert(AssetAnimator::new(waves, tween))
        .insert(StartMenu)
        .insert(Wave);

    // camera
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(StartMenu);
}

pub fn sync_with_window_size(
    mut window_size: EventReader<WindowResized>,
    handle_query: Query<&Mesh2dHandle, With<Wave>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for window_resized in window_size.iter() {
        if let Some(mesh) = handle_query
            .get_single()
            .ok()
            .and_then(|handle| meshes.get_mut(&handle.0))
        {
            *mesh = new_waves_mesh(window_resized.width, window_resized.height);
        };
    }
}

pub fn new_waves_mesh(width: f32, height: f32) -> Mesh {
    let size = Vec2::new(width, height);
    Mesh::from(shape::Quad { size, flip: false })
}

fn sync_with_time(
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

fn remove_resource(mut commands: Commands) {
    commands.remove_resource::<Offset>();
}
