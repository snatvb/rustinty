use std::os::raw::c_char;
use bevy::prelude::KeyCode;
use log::{debug, trace};
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::{types::EntityId, components::CTransform};

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new());
}

pub struct World {
    pub cb_spawn_prefab: Box<dyn Fn(*const c_char, EntityId) + Send>,
    pub cb_spawn_prefab_with_transform: Box<dyn Fn(*const c_char, EntityId, CTransform) + Send>,
    pub cb_spawn_prefab_with_transform_and_parent: Box<dyn Fn(*const c_char, EntityId, CTransform, EntityId) + Send>,
    pub cb_despawn_gameobject: Box<dyn Fn(EntityId) + Send>,
    pub cb_upload_component_transform: Box<dyn Fn(EntityId, CTransform) + Send>,
    pub cb_input_key_just_pressed: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_input_key_just_released: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_input_key_held: Box<dyn Fn(KeyCode) -> bool + Send>,
    pub cb_entity_track_collision: Box<dyn Fn(EntityId) + Send>,
}

impl World {
    pub fn new() -> Self {
        Self {
            cb_spawn_prefab: Box::new(|_,_|()),
            cb_spawn_prefab_with_transform: Box::new(|_,_,_|()),
            cb_spawn_prefab_with_transform_and_parent: Box::new(|_,_,_,_|()),
            cb_despawn_gameobject: Box::new(|_|()),
            cb_upload_component_transform: Box::new(|_,_|()),
            cb_input_key_just_pressed: Box::new(|_|false),
            cb_input_key_just_released: Box::new(|_|false),
            cb_input_key_held: Box::new(|_|false),
            cb_entity_track_collision: Box::new(|_|()),
        }
    }
}

#[no_mangle] pub extern fn rustiny_world_spawn_prefab_callback(callback: extern fn(*const c_char, EntityId)) {
    debug!("binding spawn prefab, called from C# to runity.dll");
    let mut world = WORLD.lock().unwrap();
    world.cb_spawn_prefab = Box::new(move |n, i| callback(n, i));
}

pub fn spawn_prefab(name: *const c_char, entity_id: EntityId) {
    trace!("calling spawn_prefab from game.dll to runity.dll");
    let world = WORLD.lock().unwrap();
    (world.cb_spawn_prefab)(name, entity_id);
}

