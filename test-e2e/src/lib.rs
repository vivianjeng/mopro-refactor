mopro_ffi::config!();

// circom module
#[cfg(feature = "circom")]
mod circom;
// re-export circom module to flutter rust bridge
#[cfg(feature = "circom")]
pub use circom::{
    generate_circom_proof, verify_circom_proof, CircomProof, CircomProofResult, G1, G2, ProofLib,
};

// halo2 module
#[cfg(feature = "halo2")]
mod halo2;
#[cfg(feature = "halo2")]
pub use halo2::{generate_halo2_proof, verify_halo2_proof, Halo2ProofResult};

// noir module
#[cfg(feature = "noir")]
mod noir;
#[cfg(feature = "noir")]
pub use noir::{generate_noir_proof, get_noir_verification_key, verify_noir_proof};

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
