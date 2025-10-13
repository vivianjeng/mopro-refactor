use anyhow::Context;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;

use crate::bindings::constants::{
    Mode, ReactNativeArch, ReactNativePlatform, REACT_NATIVE_BINDINGS_DIR,
};

use super::raw_project_name_from_toml;
use super::PlatformBuilder;

// Maintained for backwards compatibility
#[inline]
pub fn build() {
    super::build_from_env::<ReactNativePlatform>()
}

#[derive(Default)]
pub struct ReactNativeBindingsParams {
    pub using_noir: bool,
}

impl PlatformBuilder for ReactNativePlatform {
    type Arch = ReactNativeArch;
    type Params = ReactNativeBindingsParams;

    fn build(
        _mode: Mode,
        project_dir: &Path,
        _target_archs: Vec<Self::Arch>,
        _params: Self::Params,
    ) -> anyhow::Result<PathBuf> {
        install_uniffi_bindgen_react_native()?;

        fs::create_dir_all(project_dir.join(REACT_NATIVE_BINDINGS_DIR))
            .expect("failed to create bindings directory");
        fs::write(
            project_dir
                .join(REACT_NATIVE_BINDINGS_DIR)
                .join("ubrn.config.yaml"),
            include_str!("templates/react_native/ubrn.config.yaml").replace(
                "<%PATH_TO_PROJECT%>",
                project_dir.to_string_lossy().to_string().as_str(),
            ),
        )
        .expect("failed to write ubrn.config.yaml");
        fs::write(
            project_dir
                .join(REACT_NATIVE_BINDINGS_DIR)
                .join("package.json"),
            include_str!("templates/react_native/package.json"),
        )
        .expect("failed to write package.json");
        generate_react_native_bindings(project_dir)?;
        Ok(PathBuf::from(REACT_NATIVE_BINDINGS_DIR))
    }
}

fn install_uniffi_bindgen_react_native() -> anyhow::Result<()> {
    let output = Command::new("uniffi-bindgen-react-native").output();
    match output {
        Ok(_) => {
            // Command exists, no need to install
            println!("uniffi-bindgen-react-native already installed.");
            return Ok(());
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Command not found, proceed with installation
            println!("uniffi-bindgen-react-native not found, installing...");
            let status = Command::new("git")
                .args([
                    "clone",
                    "https://github.com/jhugman/uniffi-bindgen-react-native.git",
                ])
                .status()
                .expect("failed to download uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to download uniffi-bindgen-react-native"
                ));
            }
            let status = Command::new("cd")
                .args(["crates/ubrn_cli"])
                .status()
                .expect("failed to cd to crates/ubrn_cli");
            if !status.success() {
                return Err(anyhow::anyhow!("Failed to cd to crates/ubrn_cli"));
            }
            let status = Command::new("cargo")
                .args(["install", "--path", "."])
                .status()
                .expect("failed to install uniffi-bindgen-react-native");
            if !status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to install uniffi-bindgen-react-native"
                ));
            }
            let status = Command::new("cd")
                .args(["../.."])
                .status()
                .expect("failed to cd to ...");
            if !status.success() {
                return Err(anyhow::anyhow!("Failed to cd to ..."));
            }
            fs::remove_dir_all("uniffi-bindgen-react-native")
                .expect("failed to remove uniffi-bindgen-react-native");
        }
        Err(e) => {
            // Other error, propagate it
            return Err(anyhow::anyhow!(
                "Failed to check for uniffi-bindgen-react-native: {}",
                e
            ));
        }
    }

    Ok(())
}

fn generate_react_native_bindings(project_dir: &Path) -> anyhow::Result<()> {
    let bindings_dir = project_dir.join(REACT_NATIVE_BINDINGS_DIR);
    let status = Command::new("uniffi-bindgen-react-native")
        .args(["generate", "jsi", "turbo-module"])
        .current_dir(bindings_dir.clone())
        .status()
        .expect("failed to generate react native bindings");
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to generate react native bindings"));
    }

    let status = Command::new("uniffi-bindgen-react-native")
        .args(["build", "ios", "--and-generate", "--release"])
        .current_dir(bindings_dir)
        .status()
        .expect("failed to build react native bindings");
    if !status.success() {
        return Err(anyhow::anyhow!("Failed to build react native bindings"));
    }
    Ok(())
}
