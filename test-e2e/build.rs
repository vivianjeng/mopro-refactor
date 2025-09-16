fn main() {
    #[cfg(feature = "circom")]
    rust_witness::transpile::transpile_wasm("./test-vectors".to_string());
}
