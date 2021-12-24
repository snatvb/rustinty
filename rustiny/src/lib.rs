extern crate derive_more;
extern crate lazy_static;
pub use log::{debug, error, info, log, trace};
use std::{ffi::CString, os::raw::c_char};

pub mod components;
pub mod core;
mod game;
mod logger;
pub mod plugin;
mod synchronize;
pub mod types;
pub mod world;
pub mod prelude;

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
    unsafe { game::get_unwrap().update() };
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
