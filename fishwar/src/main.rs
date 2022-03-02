#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::{prelude::App, DefaultPlugins};
use bevy_tweening::TweeningPlugin;
use start_page::StartPagePlugin;
mod game_state;
mod start_page;
mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(StartPagePlugin)
        .add_state(game_state::FishWarState::Menu);

    #[cfg(feature = "dev")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_framepace::FramepacePlugin::default());

    app.run()
}
