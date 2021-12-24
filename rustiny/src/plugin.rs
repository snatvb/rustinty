#[macro_export]
macro_rules! register_plugin {
    ($plugin_type:ident) => {
        #[no_mangle]
        pub extern "C" fn rustiny_initialize() {
            rustiny::rustiny_initialize($plugin_type);
        }
    };
}
