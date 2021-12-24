use bevy::prelude::*;
use log::{error, trace};

use crate::{
    components::{transform::Transform, *},
    game,
    types::EntityId,
};

#[no_mangle]
pub extern "C" fn rustiny_world_sync_transform_from_unity(entity_id: EntityId, new_transform: CTransform) {
    trace!(
        "sync Transform called from Unity to rustiny.dll - id({}), CTransform - {:?}",
        entity_id,
        new_transform
    );
    let game = unsafe { game::get_unwrap() };
    if let Some(mut entity) = game.app.world.get_entity_mut(Entity::from_bits(entity_id)) {
        if let Some(mut transform_component) = entity.get_mut::<Transform>() {
            *transform_component = new_transform.into();
        } else {
            error!("entity has no Transform component");
        }
    } else {
        error!("No entity to fetch");
    }
}

pub fn sync_to_unity(query: Query<(Entity, &Transform), (With<SyncableTag>, Changed<Transform>)>) {
    for (entity, transform) in query.iter() {
        crate::world::send_component_transform(entity, transform);
    }
}
