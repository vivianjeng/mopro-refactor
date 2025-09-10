mopro_ffi::config!();
mod circom;
use num_bigint::BigInt;
use std::collections::HashMap;
use circom::multiplier2_witness;

use circom_prover::{
    prover::{
        circom::{
            Proof as CircomProverProof, CURVE_BLS12_381, CURVE_BN254, G1 as CircomProverG1,
            G2 as CircomProverG2,
        },
        ProofLib,
    },
    witness::WitnessFn,
    CircomProver,
};
use num_bigint::BigUint;
use std::str::FromStr;

// #[uniffi::export]
// fn hello_uniffi() -> String {
//     "Hello, world!".to_string()
// }

// #[cfg(target_arch = "wasm32")]
// #[wasm_bindgen]
// pub fn hello_wasm() -> String {
//     "Hello, world!".to_string()
// }

// rust_witness::witness!(multiplier2);

#[derive(Debug, Clone)]
pub struct CircomProofResult {
    pub proof: CircomProof,
    pub inputs: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CircomProof {
    pub a: G1,
    pub b: G2,
    pub c: G1,
    pub protocol: String,
    pub curve: String,
}

#[derive(Debug, Clone, Default)]
pub struct G1 {
    pub x: String,
    pub y: String,
    pub z: String,
}

#[derive(Debug, Clone, Default)]
pub struct G2 {
    pub x: Vec<String>,
    pub y: Vec<String>,
    pub z: Vec<String>,
}

impl From<CircomProverProof> for CircomProof {
    fn from(proof: CircomProverProof) -> Self {
        CircomProof {
            a: proof.a.into(),
            b: proof.b.into(),
            c: proof.c.into(),
            protocol: proof.protocol,
            curve: proof.curve,
        }
    }
}

impl From<CircomProof> for CircomProverProof {
    fn from(proof: CircomProof) -> Self {
        CircomProverProof {
            a: proof.a.into(),
            b: proof.b.into(),
            c: proof.c.into(),
            protocol: proof.protocol,
            curve: proof.curve,
        }
    }
}

impl From<CircomProverG1> for G1 {
    fn from(g1: CircomProverG1) -> Self {
        G1 {
            x: g1.x.to_string(),
            y: g1.y.to_string(),
            z: g1.z.to_string(),
        }
    }
}

impl From<G1> for CircomProverG1 {
    fn from(g1: G1) -> Self {
        CircomProverG1 {
            x: BigUint::from_str(g1.x.as_str()).unwrap(),
            y: BigUint::from_str(g1.y.as_str()).unwrap(),
            z: BigUint::from_str(g1.z.as_str()).unwrap(),
        }
    }
}

impl From<CircomProverG2> for G2 {
    fn from(g2: CircomProverG2) -> Self {
        let x = vec![g2.x[0].to_string(), g2.x[1].to_string()];
        let y = vec![g2.y[0].to_string(), g2.y[1].to_string()];
        let z = vec![g2.z[0].to_string(), g2.z[1].to_string()];
        G2 { x, y, z }
    }
}

impl From<G2> for CircomProverG2 {
    fn from(g2: G2) -> Self {
        let x =
            g2.x.iter()
                .map(|p| BigUint::from_str(p.as_str()).unwrap())
                .collect::<Vec<BigUint>>();
        let y =
            g2.y.iter()
                .map(|p| BigUint::from_str(p.as_str()).unwrap())
                .collect::<Vec<BigUint>>();
        let z =
            g2.z.iter()
                .map(|p| BigUint::from_str(p.as_str()).unwrap())
                .collect::<Vec<BigUint>>();
        CircomProverG2 {
            x: [x[0].clone(), x[1].clone()],
            y: [y[0].clone(), y[1].clone()],
            z: [z[0].clone(), z[1].clone()],
        }
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn generate_circom_proof(zkey_path: String) -> CircomProofResult {
    let inputs = std::collections::HashMap::from([
        ("a".to_string(), vec!["1".to_string()]),
        ("b".to_string(), vec!["2".to_string()]),
    ]);
    let input_str = serde_json::to_string(&inputs).unwrap();
    let proof = circom_prover::CircomProver::prove(
        circom_prover::prover::ProofLib::Arkworks,
        circom_prover::witness::WitnessFn::RustWitness(multiplier2_witness),
        input_str,
        zkey_path.to_string(),
    )
    .unwrap();
    CircomProofResult {
        proof: proof.proof.into(),
        inputs: proof.pub_inputs.into(),
    }
    // format!("Proof generated with {} values", proof.proof.a.x.to_string())
}

#[flutter_rust_bridge::frb(sync)]
pub fn verify_circom_proof(proof_result: CircomProofResult, zkey_path: String) -> bool {
    CircomProver::verify(
        circom_prover::prover::ProofLib::Arkworks,
        circom_prover::prover::CircomProof {
            proof: proof_result.proof.into(),
            pub_inputs: proof_result.inputs.into(),
        },
        zkey_path,
    ).unwrap()
}


#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hi Mopro, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
