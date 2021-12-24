use bevy::prelude::{EventWriter, ResMut};
use log::*;

use crate::types::{InputEvent, KeyCode};

#[derive(Debug, Default)]
pub struct InputBufferResource(Vec<InputEvent>);

#[no_mangle]
pub extern "C" fn rustiny_world_sync_input_from_unity(keycode: KeyCode) {
    trace!("sync input called from Unity to rustiny.dll; {}", keycode as i32);

    let game = unsafe { crate::game::get_unwrap() };
    if let Some(mut events_buffer) = game.app.world.get_resource_mut::<InputBufferResource>() {
        let event = InputEvent(keycode);
        debug!("Send event: {}", event);
        events_buffer.0.push(event);
    } else {
        error!("InputEvent non registered in bevy!");
    }
}

pub fn send_events_from_buffer(mut events: EventWriter<InputEvent>, mut events_buffer: ResMut<InputBufferResource>) {
    for event in events_buffer.0.iter() {
        events.send(*event);
    }
    events_buffer.0.clear();
}
