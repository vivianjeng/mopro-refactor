pub mod bindings;

#[cfg(feature = "uniffi")]
pub use uniffi::*;

#[cfg(feature = "uniffi")]
#[macro_export]
macro_rules! uniffi_setup {
    () => {
        // ::uniffi must be available in the caller’s extern-prelude.
        extern crate mopro_ffi as uniffi;
        uniffi::setup_scaffolding!();
    };
}

#[cfg(not(feature = "uniffi"))]
#[macro_export]
macro_rules! uniffi_setup {
    () => {};
}

// WASM-related exports when targeting WASM
#[cfg(feature = "wasm")]
pub use ::wasm_bindgen::*;

// Temporarily disabled due to WASM target compatibility issues
#[cfg(feature = "wasm")]
pub use ::wasm_bindgen_rayon::*;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
#[macro_export]
macro_rules! wasm_setup {
    () => {
        extern crate mopro_ffi as wasm_bindgen;
        extern crate mopro_ffi as wasm_bindgen_rayon;
        use wasm_bindgen::prelude::*;
        use wasm_bindgen_rayon::init_thread_pool;
    };
}

#[cfg(not(all(feature = "wasm", target_arch = "wasm32")))]
#[macro_export]
macro_rules! wasm_setup {
    () => {};
}

#[macro_export]
macro_rules! config {
    () => {
        mopro_ffi::uniffi_setup!();
        mopro_ffi::wasm_setup!();
    };
}
