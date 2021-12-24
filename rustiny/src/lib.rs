extern crate lazy_static;
pub use log::{debug, error, info, trace, log};
use std::{ffi::CString, os::raw::c_char};

pub mod components;
pub mod core;
pub mod types;
pub mod world;
pub mod plugin;
mod game;
mod logger;
mod synchronize;

#[no_mangle]
pub extern "C" fn rustiny_name() -> *mut c_char {
    CString::new("Rusty").unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rustiny_version() -> *mut c_char {
    CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn rustiny_update() {
    info!("game update");
    unsafe { game::get_unwrap().update() };
}

#[no_mangle]
pub extern "C" fn rustiny_callback(callback: extern "C" fn(u64)) {
    callback(92);
}

pub fn rustiny_initialize<T: bevy::prelude::Plugin>(game_plugin: T) {
    std::panic::set_hook(Box::new(|info| {
        error!("PANIC {:?}", info);
    }));

    debug!("Rustiny initialize");
    let game = game::Game::setup(game_plugin);
    info!("Game setup success");
    unsafe {
        game::GAME = Some(game);
    };
}
