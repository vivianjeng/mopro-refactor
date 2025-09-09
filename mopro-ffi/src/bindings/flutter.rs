use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::bindings::constants::{FlutterArch, FlutterPlatform, Mode};

use super::raw_project_name_from_toml;
use super::PlatformBuilder;

// Maintained for backwards compatibility
#[inline]
pub fn build() {
    super::build_from_env::<FlutterPlatform>()
}

#[derive(Default)]
pub struct FlutterBindingsParams {
    pub using_noir: bool,
}

impl PlatformBuilder for FlutterPlatform {
    type Arch = FlutterArch;
    type Params = FlutterBindingsParams;

    fn build(
        mode: Mode,
        project_dir: &Path,
        target_archs: Vec<Self::Arch>,
        params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        // Init flutter bindings template
        init_flutter_bindings(project_dir)?;

        // Init workspace for bindings template
        let cargo_toml_path = project_dir.join("mopro_flutter_bindings/rust/Cargo.toml");
        ensure_workspace_toml(&cargo_toml_path.to_string_lossy().to_string());

        // Import user defined crates
        let third_party_crate_name = raw_project_name_from_toml(project_dir)?;
        let cargo_add_status = Command::new("cargo")
            .args([
                "add",
                &third_party_crate_name,
                "--path",
                &project_dir.to_string_lossy().to_string(),
            ])
            .current_dir(project_dir.join("mopro_flutter_bindings/rust"))
            .status()
            .expect("failed to run cargo add");
        if !cargo_add_status.success() {
            return Err(anyhow::anyhow!("Failed to add third party crate"));
        }

        // Generate flutter bindings
        let generate_status = Command::new("flutter_rust_bridge_codegen")
            .args(["generate"])
            .args([
                "--rust-root",
                "mopro_flutter_bindings/rust",
                "--rust-input",
                "test-e2e", // crate name
                "--dart-output",
                "mopro_flutter_bindings/lib/src/rust",
            ])
            .current_dir(project_dir)
            .status()
            .expect("failed to run flutter_rust_bridge_codegen");
        if !generate_status.success() {
            return Err(anyhow::anyhow!("Failed to generate simple.rs"));
        }

        Ok(PathBuf::from("MoproDartBindings"))
    }
}

fn init_flutter_bindings(project_dir: &Path) -> anyhow::Result<()> {
    let flutter_bindings_dir = project_dir.join("mopro_flutter_bindings");
    if !flutter_bindings_dir.exists() {
        let status = Command::new("flutter_rust_bridge_codegen")
            .args(["create", "mopro_flutter_bindings", "--template", "plugin"])
            .status()
            .expect("failed to run flutter_rust_bridge_codegen");

        if !status.success() {
            return Err(anyhow::anyhow!("flutter_rust_bridge_codegen failed"));
        }
    }

    Ok(())
}

fn ensure_workspace_toml(cargo_toml_path: &str) {
    let content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

    if !content.contains("[workspace]") {
        let new_content = format!("{content}\n\n[workspace]\n");
        fs::write(cargo_toml_path, new_content).expect("Failed to write updated Cargo.toml");
    }
}
