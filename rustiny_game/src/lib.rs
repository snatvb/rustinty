extern crate rustiny;
use bevy::prelude::*;
use rustiny::components::{Transform, *};
use rustiny::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        rustiny::info!("Game creating...");
        app.add_startup_system(start.system());
        app.add_system(log_input_system.system());
        app.add_system(move_light.system());
    }
}

fn start(mut commands: Commands) {
    commands
        .spawn()
        .insert(Prefab::new("light"))
        .insert(Transform {
            position: Vector3::new(10.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        })
        .insert(SyncableTag);
}

fn log_input_system(mut events: EventReader<rustiny::types::InputEvent>) {
    for event in events.iter() {
        rustiny::info!("Pressed: {}", event);
    }
}

fn move_light(
    mut events: EventReader<rustiny::types::InputEvent>,
    mut query: Query<&mut Transform>,
    time: Res<Time>,
) {
    for event in events.iter() {
        let iter_mut = query.iter_mut();
        match event.0 {
            rustiny::types::KeyCode::LeftArrow => iter_mut
                .for_each(|mut transform| transform.position.x += 2.0 * time.delta_seconds()),
            rustiny::types::KeyCode::RightArrow => iter_mut
                .for_each(|mut transform| transform.position.x -= 2.0 * time.delta_seconds()),
            rustiny::types::KeyCode::UpArrow => iter_mut
                .for_each(|mut transform| transform.position.z -= 2.0 * time.delta_seconds()),
            rustiny::types::KeyCode::DownArrow => iter_mut
                .for_each(|mut transform| transform.position.z += 2.0 * time.delta_seconds()),
            _ => (),
        }
    }
}

rustiny::register_plugin!(GamePlugin);
