use crate::types::InputEvent;
use bevy::prelude::*;

mod input;
mod prefab;
mod transform;

pub struct SynchronizePlugin;

impl Plugin for SynchronizePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<InputEvent>();
        app.insert_resource(input::InputBufferResource::default());
        app.add_system_to_stage(CoreStage::First, input::send_events_from_buffer.exclusive_system());
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            transform::sync_to_unity.exclusive_system().label("sync_to_unity")
            .after("sync_spawn_to_unity"),
        );
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            prefab::sync_spawn_to_unity
                .exclusive_system()
                .label("sync_spawn_to_unity"),
        );
    }
}
