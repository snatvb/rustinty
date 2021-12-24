use bevy::{prelude::{KeyCode, Entity}};
use lazy_static::lazy_static;
use log::{debug, trace};
use std::os::raw::c_char;
use std::sync::Mutex;

use crate::{components::{CTransform, Transform}, types::EntityId};

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new());
}

pub struct World {
    pub spawn_prefab: Box<dyn Fn(*const c_char, EntityId) + Send>,
    pub cb_spawn_prefab_with_transform: Box<dyn Fn(*const c_char, EntityId, CTransform) + Send>,
    pub cb_spawn_prefab_with_transform_and_parent:
        Box<dyn Fn(*const c_char, EntityId, CTransform, EntityId) + Send>,
    pub cb_despawn_gameobject: Box<dyn Fn(EntityId) + Send>,
    pub send_component_transform: Box<dyn Fn(EntityId, CTransform) + Send>,
    pub cb_input_key_just_pressed: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_input_key_just_released: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_input_key_held: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_entity_track_collision: Box<dyn Fn(EntityId) + Send>,
}

impl World {
    pub fn new() -> Self {
        Self {
            spawn_prefab: Box::new(|_, _| ()),
            cb_spawn_prefab_with_transform: Box::new(|_, _, _| ()),
            cb_spawn_prefab_with_transform_and_parent: Box::new(|_, _, _, _| ()),
            cb_despawn_gameobject: Box::new(|_| ()),
            send_component_transform: Box::new(|_, _| ()),
            cb_input_key_just_pressed: Box::new(|_| false),
            cb_input_key_just_released: Box::new(|_| false),
            cb_input_key_held: Box::new(|_| false),
            cb_entity_track_collision: Box::new(|_| ()),
        }
    }
}

#[no_mangle]
pub extern "C" fn rustiny_world_spawn_prefab_callback(
    callback: extern "C" fn(*const c_char, EntityId),
) {
    debug!("binding spawn prefab, called from Unity to rustiny.dll");
    let mut world = WORLD.lock().unwrap();
    world.spawn_prefab = Box::new(move |n, i| callback(n, i));
}

pub fn spawn_prefab(name: *const c_char, entity_id: EntityId) {
    trace!("calling spawn_prefab from rustiny.dll to Unity");
    let world = WORLD.lock().unwrap();
    (world.spawn_prefab)(name, entity_id);
}

#[no_mangle] pub extern fn rustiny_world_sync_transform_bind(callback: extern fn(EntityId, CTransform)) {
    debug!("binding upload component transform, called from Unity to rustiny.dll");
    let mut world = WORLD.lock().unwrap();
    world.send_component_transform = Box::new(move |id, transform| callback(id, transform));
}

pub fn send_component_transform(entity: Entity, transform: Transform) {
    trace!("calling send_component_transform from rustiny.dll to Unity");
    let world = WORLD.lock().unwrap();
    (world.send_component_transform)(entity.to_bits(), transform.into());
}
