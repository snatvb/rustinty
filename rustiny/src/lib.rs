extern crate lazy_static;
use bevy::prelude::*;
pub use log::{debug, error, info, trace, log};
use std::{ffi::CString, os::raw::c_char};

pub mod components;
pub mod core;
pub mod types;
pub mod world;
pub mod plugin;
mod logger;

// DANGEROUS PLACE
#[export_name = "__rustiny_game__"]
pub static mut GAME: Option<Game> = None;

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
    if let Some(game) = unsafe { &mut GAME } {
        info!("game update");
        game.update();
    }
}

#[no_mangle]
pub extern "C" fn rustiny_callback(callback: extern "C" fn(u64)) {
    callback(92);
}

pub fn rustiny_initialize<T: bevy::prelude::Plugin>(game_plugin: T) {
    info!("Game creating...");
    let game = Game::setup(game_plugin);
    unsafe {
        GAME = Some(game);
    };
}

trait GameApi {
    fn update(&mut self);
}

pub struct Game {
    app: App,
}

impl GameApi for Game {
    fn update(&mut self) {
        #[cfg(feature = "trace")]
        let bevy_frame_update_span = info_span!("frame");
        #[cfg(feature = "trace")]
        let _bevy_frame_update_guard = bevy_frame_update_span.enter();
        self.app.schedule.run(&mut self.app.world);
    }
}

fn runner(mut app: App) {
    app.update();
}

fn rustiny_test_system() {
    debug!("rustiny_test_system");
}

impl Game {
    pub fn setup<T: bevy::prelude::Plugin>(game_plugin: T) -> Self {
        let mut app = App::build();
        debug!("App building...");
        let mut time = Time::default();

        time.update();
        app.insert_resource(time);
        app.add_system_to_stage(
            CoreStage::First,
            (|mut time: ResMut<Time>| time.update()).exclusive_system(),
        );

        app.add_plugin(game_plugin);
        app.set_runner(runner);
        app.add_system_to_stage(CoreStage::First, rustiny_test_system.system());
        Self { app: app.app }
    }
}
