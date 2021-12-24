use bevy::prelude::*;
use log::trace;

use crate::components::{Transform, *};

pub fn sync_spawn_to_unity(query: Query<(Entity, &Prefab, &Transform), (Added<Prefab>, Without<Parent>)>) {
    for (entity, prefab, transform) in query.iter() {
        trace!(
            "telling unity to spawn prefab({}), id {} with {:?}",
            prefab.name,
            entity.to_bits(),
            transform
        );
        crate::world::spawn_prefab(entity.to_bits(), &prefab.name, transform);
    }
}
