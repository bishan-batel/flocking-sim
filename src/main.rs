use bevy::prelude::*;
use plugins::flockplugin::FlockPlugin;

mod plugins;
mod test_plugin;
mod utils;

// rofl bob
fn main() {
    let window_description = WindowDescriptor {
        title: String::from("Rofl bob"),
        width: 800.0,
        height: 800.0,
        vsync: false,
        resizable: false,
        ..Default::default()
    };

    App::build()
        .insert_resource(window_description)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.8)))
        // .add_plugin(PlayerPlugin)

        .add_plugin(FlockPlugin)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
