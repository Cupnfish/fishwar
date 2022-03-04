use bevy::{prelude::*, window::WindowCloseRequested};

use crate::{game_state::FishWarState, utils::despawn_screen};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(FishWarState::GameOver).with_system(setup))
            .add_system_set(
                SystemSet::on_update(FishWarState::GameOver)
                    .with_system(quit_game)
                    .with_system(reopen_game),
            )
            .add_system_set(
                SystemSet::on_exit(FishWarState::GameOver)
                    .with_system(despawn_screen::<GameOverDespawn>),
            );
    }
}
#[derive(Component)]
struct GameOverDespawn;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(GameOverDespawn);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(10.0),
                    left: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..Default::default()
        })
        .insert(GameOverDespawn)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Game Over! Have Fun? \n Please press 'ecs' button or 'Q'\n button to exit the game.\n Or 'M' button to go back\n to the menu and 'G' button to restart\n the game.\n Did you find the 'space' button\n in the game can trigger the Unfair\n Advantage?",
                    TextStyle {
                        font: asset_server.load("fonts/finger-paint-regular.ttf"),
                        font_size: 80.0,
                        color: Color::CRIMSON,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

fn quit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut quit: EventWriter<WindowCloseRequested>,
    windows: Res<Windows>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) || keyboard_input.just_pressed(KeyCode::Escape) {
        quit.send(WindowCloseRequested {
            id: windows.get_primary().unwrap().id(),
        })
    }
}

fn reopen_game(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<State<FishWarState>>) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if let Err(e) = game_state.set(FishWarState::Menu) {
            warn!("set state error: {:?}", e);
        };
    }
    if keyboard_input.just_pressed(KeyCode::G) {
        if let Err(e) = game_state.set(FishWarState::Game) {
            warn!("set state error: {:?}", e);
        };
    }
}
