mopro_ffi::config!();

// #[uniffi::export]
// fn hello_uniffi() -> String {
//     "Hello, world!".to_string()
// }

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub fn hello_wasm() -> String {
//     "Hello, world!".to_string()
// }

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hi Mopro, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
