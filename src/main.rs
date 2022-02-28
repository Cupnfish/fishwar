use bevy::{prelude::App, DefaultPlugins};
use bevy_tweening::TweeningPlugin;
use start_page::StartPagePlugin;
mod start_page;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(TweeningPlugin)
        .add_plugin(StartPagePlugin);

    #[cfg(feature = "dev")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
        .add_plugin(bevy_framepace::FramepacePlugin::default());

    app.run()
}
