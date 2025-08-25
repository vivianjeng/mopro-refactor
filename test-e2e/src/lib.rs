mopro_ffi::config!();

#[uniffi::export]
fn hello_uniffi() -> String {
    "Hello, world!".to_string()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn hello_wasm() -> String {
    "Hello, world!".to_string()
}
