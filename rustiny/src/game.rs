use bevy::prelude::*;

// DANGEROUS PLACE
#[export_name = "__rustiny_game__"]
pub static mut GAME: Option<Game> = None;

pub struct Game {
    pub app: App,
}

fn runner(mut app: App) {
    app.update();
}

fn rustiny_test_system(query: Query<Entity>) {
    debug!("rustiny_test_system items {}", query.iter().count());
}

impl Game {
    pub fn setup<T: bevy::prelude::Plugin>(game_plugin: T) -> Self {
        debug!("App building...");
        let mut app = App::build();
        let mut time = Time::default();

        time.update();
        app.insert_resource(time);
        app.add_system_to_stage(CoreStage::First, (|mut time: ResMut<Time>| time.update()).exclusive_system());

        app.add_plugin(crate::synchronize::SynchronizePlugin);

        debug!("Add game plugin");
        app.add_plugin(game_plugin);
        app.set_runner(runner);
        app.add_system_to_stage(CoreStage::First, rustiny_test_system.system());
        Self { app: app.app }
    }

    pub fn update(&mut self) {
        #[cfg(feature = "trace")]
        let bevy_frame_update_span = info_span!("frame");
        #[cfg(feature = "trace")]
        let _bevy_frame_update_guard = bevy_frame_update_span.enter();
        self.app.schedule.run(&mut self.app.world);
    }
}

pub unsafe fn get_unwrap() -> &'static mut Game {
    #[cfg(debug_assertions)]
    if GAME.is_none() {
        error!("Try to get game before initialization!")
    }

    GAME.as_mut().unwrap()
}
