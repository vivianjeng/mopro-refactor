mopro_ffi::config!();

// circom module
mod circom;
// re-export circom module to flutter rust bridge
pub use circom::{
    generate_circom_proof, verify_circom_proof, CircomProof, CircomProofResult, G1, G2,
};

// #[uniffi::export]
// fn hello_uniffi() -> String {
//     "Hello, world!".to_string()
// }

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub fn hello_wasm() -> String {
//     "Hello, world!".to_string()
// }

pub fn greet(name: String) -> String {
    format!("Hi Mopro, {name}!")
}


