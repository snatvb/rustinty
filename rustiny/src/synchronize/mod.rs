use bevy::prelude::*;

// use crate::core::AppStage;

pub mod transform;

pub struct SynchronizePlugin;

impl Plugin for SynchronizePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(CoreStage::PostUpdate, transform::sync_to_unity.exclusive_system());
    }
}
