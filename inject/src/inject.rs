use std::time::Duration;

use bevy::window::WindowResized;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use bevy_tweening::AssetAnimator;
use bevy_tweening::{
    component_animator_system, lens::TransformScaleLens, Animator, EaseFunction, EaseMethod, Lens,
    Lerp, Tween, TweeningType,
};
use heron::prelude::*;
use rand::Rng;

use crate::start_page::Wave;
use crate::{
    game_state::FishWarState,
    start_page::new_waves_mesh,
    utils::despawn_screen,
    waves::{WavesMaterial, WavesPropertiesLens},
};

pub struct InjectPluge;

impl Plugin for InjectPluge {
    fn build(&self, app: &mut App) {
        app.add_event::<Source>()
            .add_system_set(SystemSet::on_enter(FishWarState::Game).with_system(setup))
            .add_system_set(
                SystemSet::on_update(FishWarState::Game)
                    .with_system(space_to_unfair)
                    .with_system(handle_inject)
                    .with_system(sync_with_window_size)
                    .with_system(sync_mouse_postion)
                    .with_system(sync_with_time)
                    .with_system(gen_new_inject)
                    .with_system(component_animator_system::<CollisionShape>)
                    .with_system(crate::start_page::sync_with_window_size),
            )
            .add_system_set(
                SystemSet::on_exit(FishWarState::Game)
                    .with_system(despawn_screen::<InjecDespawn>)
                    .with_system(remove_resource),
            );
    }
}
#[derive(Component)]
struct InjecDespawn;

#[derive(Component)]
struct Inject;

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct InjectCamera;

struct EnoughRadius(f32);

impl Default for EnoughRadius {
    fn default() -> Self {
        Self(50.0)
    }
}

const DEFAULT_WALL_WIDTH: f32 = 0.1;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WavesMaterial>>,
) {
    commands.insert_resource(CurrentInject::default());
    commands.insert_resource(MaxInject::default());
    commands.insert_resource(InitRadius::default());
    commands.insert_resource(EnoughRadius::default());

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(InjectCamera)
        .insert(InjecDespawn);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("icon.png"),
            transform: Transform::from_xyz(1000., 200., -1.),
            ..Default::default()
        })
        .insert(InjecDespawn);

    let window = windows.get_primary().unwrap();
    let half_width = window.width() * 0.5;
    let half_height = window.height() * 0.5;

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
        .insert(InjecDespawn)
        .insert(Wave);

    commands.spawn_bundle((
        InjecDespawn,
        GlobalTransform::default(),
        Transform::default(),
        Mouse,
        RigidBody::Sensor,
        CollisionShape::Sphere { radius: 0.5 },
        CollisionLayers::new(Layer::Mouse, Layer::Inject),
    ));
    let init_radius = InitRadius::default().0;

    spawn_inject(
        &mut commands,
        CurrentInject::default().0,
        init_radius,
        half_width - init_radius,
        half_height - init_radius,
        &asset_server,
    );

    spawn_all_wall(&mut commands, window.width(), window.height());
}

fn spawn_all_wall(commands: &mut Commands, window_width: f32, window_helight: f32) {
    let half_width = window_width * 0.5;
    let half_height = window_helight * 0.5;

    spawn_wall(
        commands,
        Vec2::new(DEFAULT_WALL_WIDTH, window_helight),
        Transform::from_xyz(half_width, 0.0, 1.0),
    );

    spawn_wall(
        commands,
        Vec2::new(DEFAULT_WALL_WIDTH, window_helight),
        Transform::from_xyz(-half_width, 0.0, 1.0),
    );

    spawn_wall(
        commands,
        Vec2::new(window_width, DEFAULT_WALL_WIDTH),
        Transform::from_xyz(0.0, half_height, 1.0),
    );

    spawn_wall(
        commands,
        Vec2::new(window_width, DEFAULT_WALL_WIDTH),
        Transform::from_xyz(0.0, -half_height, 1.0),
    );
}

struct CurrentInject(u8);

impl Default for CurrentInject {
    fn default() -> Self {
        Self(3)
    }
}

struct MaxInject(u8);

impl Default for MaxInject {
    fn default() -> Self {
        Self(4)
    }
}

struct InitRadius(f32);

impl Default for InitRadius {
    fn default() -> Self {
        Self(20.)
    }
}

fn spawn_inject(
    commands: &mut Commands,
    num: u8,
    radius: f32,
    half_width: f32,
    half_height: f32,
    asset_server: &AssetServer,
) {
    for _ in 0..num {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("icon.png"),
                transform: Transform::from_xyz(
                    rand::thread_rng().gen_range(-half_width..half_width),
                    rand::thread_rng().gen_range(-half_height..half_height),
                    1.,
                ),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(radius * 2.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle((
                InjecDespawn,
                Inject,
                RigidBody::Dynamic,
                CollisionShape::Sphere { radius },
                Velocity::from_linear(Vec3::new(
                    rand_f32_for_velocity(),
                    rand_f32_for_velocity(),
                    rand_f32_for_velocity(),
                ))
                .with_angular(AxisAngle::new(Vec3::Z, rand_f32_for_angular())),
                PhysicMaterial {
                    restitution: 0.7,
                    ..Default::default()
                },
                CollisionLayers::none()
                    .with_group(Layer::Inject)
                    .with_masks(&[Layer::Inject, Layer::Wall, Layer::Mouse]),
            ));
    }
}

fn gen_new_inject(
    mut commands: Commands,
    mut current_num: ResMut<CurrentInject>,
    mut max_num: ResMut<MaxInject>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
    init_radois: Res<InitRadius>,
) {
    if current_num.0 == 0 {
        let window = windows.get_primary().unwrap();
        let half_width = window.width() * 0.5 - init_radois.0;
        let half_height = window.height() * 0.5 - init_radois.0;
        spawn_inject(
            &mut commands,
            max_num.0,
            init_radois.0,
            half_width,
            half_height,
            &asset_server,
        );
        current_num.0 = max_num.0;
        if let Some(res) = max_num.0.checked_add(1) {
            max_num.0 = res;
        }
    }
}

fn rand_f32_for_velocity() -> f32 {
    let res = rand::thread_rng().gen_range(50.0..80.0);
    if rand::random() {
        res
    } else {
        -res
    }
}
fn rand_f32_for_angular() -> f32 {
    let res = rand::thread_rng().gen_range(0.3..1.2);
    if rand::random() {
        res
    } else {
        -res
    }
}
fn spawn_wall(commands: &mut Commands, size: Vec2, position: Transform) {
    commands.spawn_bundle((
        InjecDespawn,
        position,
        GlobalTransform::default(),
        Wall,
        RigidBody::Static,
        CollisionShape::Cuboid {
            half_extends: size.extend(0.0) / 2.0,
            border_radius: None,
        },
        PhysicMaterial {
            restitution: 0.5,
            ..Default::default()
        },
        CollisionLayers::new(Layer::Wall, Layer::Inject),
    ));
}

fn sync_with_window_size(
    mut commands: Commands,
    mut resize: EventReader<WindowResized>,
    wall_query: Query<Entity, With<Wall>>,
    mut query_inject: Query<(&mut Transform, &Sprite), With<Inject>>,
) {
    if let Some(resize) = resize.iter().last() {
        let half_resize_width = resize.width * 0.5;
        let half_resize_height = resize.height * 0.5;
        for (mut transform, sprite) in query_inject.iter_mut() {
            let radius = sprite.custom_size.unwrap().x;
            let half_width = half_resize_width - radius;
            let half_height = half_resize_height - radius;

            if transform.translation.x > half_width {
                transform.translation.x = half_width;
            }

            if transform.translation.x < -half_width {
                transform.translation.x = -half_width;
            }

            if transform.translation.y > half_height {
                transform.translation.x = half_height;
            }

            if transform.translation.y < -half_height {
                transform.translation.x = -half_height;
            }
        }

        spawn_all_wall(&mut commands, resize.width, resize.height);

        for wall in wall_query.iter() {
            commands.entity(wall).despawn();
        }
    }
}

fn screen_to_world_line<W: AsRef<Windows>>(
    pos_screen: Vec2,
    windows: W,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Line {
    let camera_position = camera_transform.compute_matrix();
    let window = windows
        .as_ref()
        .get(camera.window)
        .unwrap_or_else(|| panic!("WindowId {} does not exist", camera.window));
    let screen_size = Vec2::from([window.width() as f32, window.height() as f32]);
    let projection_matrix = camera.projection_matrix;

    // Normalized device coordinate cursor position from (-1, -1, -1) to (1, 1, 1)
    let cursor_ndc = (pos_screen / screen_size) * 2.0 - Vec2::from([1.0, 1.0]);
    let cursor_pos_ndc_near: Vec3 = cursor_ndc.extend(-1.0);
    let cursor_pos_ndc_far: Vec3 = cursor_ndc.extend(1.0);

    // Use near and far ndc points to generate a ray in world space
    // This method is more robust than using the location of the camera as the start of
    // the ray, because ortho cameras have a focal point at infinity!
    let ndc_to_world: Mat4 = camera_position * projection_matrix.inverse();
    let cursor_pos_near: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_near);
    let cursor_pos_far: Vec3 = ndc_to_world.project_point3(cursor_pos_ndc_far);
    let ray_direction = cursor_pos_far - cursor_pos_near;
    Line::from_point_direction(cursor_pos_near, ray_direction)
    //Ray3d::new(cursor_pos_near, ray_direction)
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Line {
    point: Vec3,
    direction: Vec3,
}
impl Line {
    pub fn from_point_direction(point: Vec3, direction: Vec3) -> Self {
        Line { point, direction }
    }
}

/// Given a position in screen space and a plane in world space, compute what point on the plane the point in screen space corresponds to.
/// In 2D, use `screen_to_point_2d`.
fn screen_to_point_on_plane<W: AsRef<Windows>>(
    pos_screen: Vec2,
    plane: Plane,
    windows: W,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    let world_line = screen_to_world_line(pos_screen, windows, camera, camera_transform);
    plane.intersection_line(&world_line)
}

/// Computes the world position for a given screen position.
/// The output will always be on the XY plane with Z at zero. It is designed for 2D, but also works with a 3D camera.
/// For more flexibility in 3D, consider `screen_to_point_on_plane`.
fn screen_to_point_2d<W: AsRef<Windows>>(
    pos_screen: Vec2,
    windows: W,
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec3> {
    screen_to_point_on_plane(
        pos_screen,
        Plane::from_point_normal(Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.)),
        windows,
        camera,
        camera_transform,
    )
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Plane {
    point: Vec3,
    normal: Vec3,
}
impl Plane {
    /// Generate a plane from a point on that plane and the normal direction of the plane. The
    /// normal vector does not need to be normalized (length can be != 1).
    fn from_point_normal(point: Vec3, normal: Vec3) -> Plane {
        Plane {
            point,
            normal: normal.normalize(),
        }
    }

    /// Compute the intersection of the plane and a line.
    /// Returns None if the plane and line are parallel.
    fn intersection_line(&self, line: &Line) -> Option<Vec3> {
        let d = line.direction.dot(self.normal);
        if d == 0. {
            // Should probably check if they're approximately equal, not strictly equal
            None
        } else {
            let diff = line.point - self.point;
            let p = diff.dot(self.normal);
            let dist = p / d;
            Some(line.point - line.direction * dist)
        }
    }
}

#[derive(Component)]
struct Mouse;

fn sync_mouse_postion(
    windows: Res<Windows>,
    mut mouse_query: Query<&mut Transform, With<Mouse>>,
    camera: Query<(&Camera, &GlobalTransform), With<InjectCamera>>,
) {
    if let Some(position) = windows.get_primary().and_then(|w| w.cursor_position()) {
        let (camera, global_transform) = camera.get_single().unwrap();
        if let Some(point) = screen_to_point_2d(position, windows, camera, global_transform) {
            let mut transform = mouse_query.get_single_mut().unwrap();
            *transform = Transform::from_translation(point);
        }
    }
}

fn handle_inject(
    mut commands: Commands,
    mut events: EventReader<CollisionEvent>,
    inject_query: Query<(&Transform, &CollisionShape, Option<&Animator<Transform>>), With<Inject>>,
    enough: Res<EnoughRadius>,
    mut current: ResMut<CurrentInject>,
    mut source: EventWriter<Source>,
) {
    for inject_entity in events
        .iter()
        // We care about when the entities "start" to collide
        // .filter(|e| e.is_started())
        .filter_map(|event| {
            let (entity_1, entity_2) = event.rigid_body_entities();
            let (layers_1, layers_2) = event.collision_layers();
            if is_mouse(layers_1) && is_inject(layers_2) {
                Some(entity_2)
            } else if is_mouse(layers_2) && is_inject(layers_1) {
                Some(entity_1)
            } else {
                // This event is not the collision between an enemy and the player. We can ignore it.
                None
            }
        })
    {
        if let Ok((transform, shape, op_t)) = inject_query.get(inject_entity) {
            if is_shape_enough(shape, enough.0) {
                if let Some(res) = current.0.checked_sub(1) {
                    current.0 = res;
                    source.send(Source);
                    commands.entity(inject_entity).despawn();
                }
                continue;
            }

            if let Some(t) = op_t {
                if t.progress() < 1.0 {
                    continue;
                }
            }

            let rand = rand::thread_rng().gen_range(1.5..3.6);
            let start = transform.scale;
            let end = start * rand;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                TweeningType::Once,
                Duration::from_secs_f32(1.0),
                TransformScaleLens { start, end },
            );

            if let CollisionShape::Sphere { radius } = shape {
                let collision_tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    TweeningType::Once,
                    Duration::from_secs_f32(1.0),
                    CollisionShapeSacleLens {
                        start: *radius,
                        end: *radius * rand,
                    },
                );
                commands
                    .entity(inject_entity)
                    .insert(Animator::new(collision_tween))
                    .insert(Animator::new(tween));
            }
        }
    }
}

fn is_shape_enough(shape: &CollisionShape, enough: f32) -> bool {
    if let CollisionShape::Sphere { radius } = shape {
        return *radius >= enough;
    }
    false
}

fn is_mouse(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Mouse)
}

fn is_inject(layers: CollisionLayers) -> bool {
    layers.contains_group(Layer::Inject)
}

#[derive(PhysicsLayer)]
enum Layer {
    Mouse,
    Wall,
    Inject,
}

struct CollisionShapeSacleLens {
    start: f32,
    end: f32,
}

impl Lens<CollisionShape> for CollisionShapeSacleLens {
    fn lerp(&mut self, target: &mut CollisionShape, ratio: f32) {
        if let CollisionShape::Sphere { radius } = target {
            *radius = self.start.lerp(&self.end, &ratio);
        }
    }
}

fn sync_with_time(
    mut materials: ResMut<Assets<WavesMaterial>>,
    query_waves: Query<&Handle<WavesMaterial>>,
    time: Res<Time>,
    mut source: EventReader<Source>,
    mut game_state: ResMut<State<FishWarState>>,
) {
    let handle = query_waves.get_single().unwrap();
    let waves = materials.get_mut(handle).unwrap();

    waves.time = time.seconds_since_startup() as f32;

    if waves.offset <= 0.0 {
        if let Err(e) = game_state.set(FishWarState::GameOver) {
            warn!("set state error: {:?}", e);
        };
    }

    for _ in source.iter() {
        waves.offset -= rand::thread_rng().gen_range(0.01..0.02);
    }
}

fn space_to_unfair(
    mut commands: Commands,
    inject_query: Query<
        (
            Entity,
            &Transform,
            &CollisionShape,
            Option<&Animator<Transform>>,
        ),
        With<Inject>,
    >,
    enough: Res<EnoughRadius>,
    mut current: ResMut<CurrentInject>,
    mut source: EventWriter<Source>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (inject_entity, transform, shape, op_t) in inject_query.iter() {
            if is_shape_enough(shape, enough.0) {
                if let Some(res) = current.0.checked_sub(1) {
                    current.0 = res;
                    source.send(Source);
                    commands.entity(inject_entity).despawn();
                }
                continue;
            }

            if let Some(t) = op_t {
                if t.progress() < 1.0 {
                    continue;
                }
            }

            let rand = rand::thread_rng().gen_range(1.5..3.6);
            let start = transform.scale;
            let end = start * rand;
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                TweeningType::Once,
                Duration::from_secs_f32(1.0),
                TransformScaleLens { start, end },
            );

            if let CollisionShape::Sphere { radius } = shape {
                let collision_tween = Tween::new(
                    EaseFunction::QuadraticInOut,
                    TweeningType::Once,
                    Duration::from_secs_f32(1.0),
                    CollisionShapeSacleLens {
                        start: *radius,
                        end: *radius * rand,
                    },
                );
                commands
                    .entity(inject_entity)
                    .insert(Animator::new(collision_tween))
                    .insert(Animator::new(tween));
            }
        }
    }
}

struct Source;

fn remove_resource(mut commands: Commands) {
    commands.remove_resource::<CurrentInject>();
    commands.remove_resource::<MaxInject>();
    commands.remove_resource::<InitRadius>();
    commands.remove_resource::<EnoughRadius>();
}
