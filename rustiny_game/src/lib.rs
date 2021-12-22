use std::{os::raw::c_char, ffi::CString};
extern crate rustiny;
use bevy::prelude::*;


#[no_mangle]
pub extern "C" fn hello_world() -> *mut c_char {
    CString::new(format!("Hello from game!")).unwrap().into_raw()
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        rustiny::info!("Game creating...");
        app.add_system(test_system.system());
    }
}

fn test_system(time: Res<Time>) {
    rustiny::debug!("delta time: {}s", time.delta_seconds());
}

rustiny::register_plugin!(GamePlugin);
