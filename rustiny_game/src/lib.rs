extern crate rustiny;
use bevy::prelude::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        rustiny::info!("Game creating...");
        // app.add_system(test_system.system());
    }
}

fn test_system(time: Res<Time>) {
    rustiny::debug!("delta time: {}s", time.delta_seconds());
}

rustiny::register_plugin!(GamePlugin);
