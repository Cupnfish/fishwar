#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::type_complexity)]

use bevy::{prelude::App, DefaultPlugins};
use bevy_tweening::TweeningPlugin;
use game_over::GameOverPlugin;
use inject::InjectPluge;
use start_page::StartPagePlugin;
use waves::WavesPlugin;

mod game_over;
mod game_state;
mod inject;
mod start_page;
mod utils;
mod waves;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(StartPagePlugin)
        .add_plugin(InjectPluge)
        .add_plugin(WavesPlugin)
        .add_plugin(GameOverPlugin)
        .add_plugin(heron::prelude::PhysicsPlugin::default())
        .add_state(game_state::FishWarState::Menu);

    #[cfg(feature = "dev")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_framepace::FramepacePlugin::default());

    app.run()
}
