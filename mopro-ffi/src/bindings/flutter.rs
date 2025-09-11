use anyhow::Context;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;

use crate::bindings::constants::{
    Arch, FlutterArch, FlutterPlatform, Mode, ARCH_ARM_64, ARCH_X86_64, FLUTTER_BINDINGS_DIR,
};
use crate::bindings::{install_arch, mktemp_local, project_name_from_toml};

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
        let cargo_toml_path = project_dir
            .join(FLUTTER_BINDINGS_DIR)
            .join("rust/Cargo.toml");
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
            .current_dir(project_dir.join(FLUTTER_BINDINGS_DIR).join("rust"))
            .status()
            .expect("failed to run cargo add");
        if !cargo_add_status.success() {
            return Err(anyhow::anyhow!("Failed to add third party crate"));
        }

        // Replace relative path with absolute path
        replace_relative_path_with_absolute(
            &cargo_toml_path,
            &third_party_crate_name,
            &project_dir,
        )?;

        // lipo the libraries together
        let lib_identifier = project_name_from_toml(project_dir)
            .expect("Failed to get project name from Cargo.toml");
        let lib_name = format!("lib{lib_identifier}.a");
        let build_dir_path: PathBuf = project_dir.join("build");
        let work_dir = mktemp_local(&build_dir_path);
        // Take a list of architectures, build them, and combine them into
        // a single universal binary/archive
        let build_combined_archs = |archs: &[FlutterArch]| -> PathBuf {
            let out_lib_paths: Vec<PathBuf> = archs
                .iter()
                .map(|arch| {
                    Path::new(&build_dir_path).join(format!(
                        "{}/{}/{}",
                        arch.as_str(),
                        mode.as_str(),
                        lib_name
                    ))
                })
                .collect();
            for arch in archs {
                install_arch(arch.as_str().to_string());
                let mut build_cmd = Command::new("cargo");
                build_cmd.arg("build");
                if mode == Mode::Release {
                    build_cmd.arg("--release");
                }
                // The dependencies of Noir libraries need iOS 15 and above.
                if params.using_noir {
                    build_cmd.env("IPHONEOS_DEPLOYMENT_TARGET", "15.0");
                }
                build_cmd
                    .arg("--lib")
                    .env("CARGO_BUILD_TARGET_DIR", &build_dir_path)
                    .env("CARGO_BUILD_TARGET", arch.as_str())
                    .spawn()
                    .expect("Failed to spawn cargo build")
                    .wait()
                    .expect("cargo build errored");
            }
            // now lipo the libraries together
            let mut lipo_cmd = Command::new("lipo");
            let lib_out = mktemp_local(&build_dir_path).join(lib_name.clone());
            lipo_cmd
                .arg("-create")
                .arg("-output")
                .arg(lib_out.to_str().unwrap());
            for p in out_lib_paths {
                lipo_cmd.arg(p.to_str().unwrap());
            }
            lipo_cmd
                .spawn()
                .expect("Failed to spawn lipo")
                .wait()
                .expect("lipo command failed");

            lib_out
        };

        let out_lib_paths: Vec<PathBuf> = group_target_archs(&target_archs)
            .iter()
            .map(|v| build_combined_archs(v))
            .collect();

        let out_dylib_path = build_dir_path.join(format!(
            "{}/{}/{}",
            target_archs[0].as_str(),
            mode.as_str(),
            lib_name.replace(".a", ".dylib")
        ));

        // Generate flutter bindings
        let rust_root = project_dir.join(FLUTTER_BINDINGS_DIR).join("rust");
        let dart_output = project_dir.join(FLUTTER_BINDINGS_DIR).join("lib/src/rust");
        let generate_status = Command::new("flutter_rust_bridge_codegen")
            .args(["generate"])
            .args([
                "--rust-root",
                &rust_root.to_string_lossy(),
                "--rust-input",
                &third_party_crate_name,
                "--dart-output",
                &dart_output.to_string_lossy(),
            ])
            .current_dir(project_dir)
            .status()
            .expect("failed to run flutter_rust_bridge_codegen");
        if !generate_status.success() {
            return Err(anyhow::anyhow!("Failed to generate simple.rs"));
        }

        Ok(PathBuf::from(FLUTTER_BINDINGS_DIR))
    }
}

fn init_flutter_bindings(project_dir: &Path) -> anyhow::Result<()> {
    let flutter_bindings_dir = project_dir.join(FLUTTER_BINDINGS_DIR);
    // TODO: install flutter_rust_bridge_codegen if not exists
    if !flutter_bindings_dir.exists() {
        let status = Command::new("flutter_rust_bridge_codegen")
            .args(["create", FLUTTER_BINDINGS_DIR, "--template", "plugin"])
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

fn replace_relative_path_with_absolute(
    cargo_toml_path: &Path,
    crate_name: &str,
    abs_path: &Path,
) -> anyhow::Result<()> {
    let cargo_toml_content =
        fs::read_to_string(cargo_toml_path).context("Failed to read Cargo.toml")?;
    let mut cargo_toml: Value = cargo_toml_content
        .parse::<Value>()
        .context("Failed to parse Cargo.toml")?;

    // If the `name` under [lib] section is set, using the `name` as library name.
    // Otherwise, using the package name.
    let crate_path = cargo_toml
        .get_mut("dependencies")
        .and_then(|pkg| pkg.get_mut(crate_name));
    // .and_then(|pkg| pkg.as_str().map(|s| s.to_string()));

    if let Some(Value::Table(table)) = crate_path {
        table.insert(
            "path".to_string(),
            Value::String(abs_path.to_string_lossy().to_string()),
        );
    }

    let updated_cargo_toml_content =
        toml::to_string_pretty(&cargo_toml).context("Failed to serialize updated Cargo.toml")?;

    fs::write(cargo_toml_path, updated_cargo_toml_content)
        .context("Failed to write updated Cargo.toml")?;

    Ok(())
}

// More general cases
fn group_target_archs(target_archs: &[FlutterArch]) -> Vec<Vec<FlutterArch>> {
    // Detect the current architecture
    let current_arch = std::env::consts::ARCH;

    // Determine the device architecture prefix based on the current architecture
    let device_prefix = match current_arch {
        arch if arch.starts_with(ARCH_X86_64) => ARCH_X86_64,
        arch if arch.starts_with(ARCH_ARM_64) => ARCH_ARM_64,
        _ => panic!("Unsupported host architecture: {current_arch}"),
    };

    let mut device_archs = Vec::new();
    let mut simulator_archs = Vec::new();

    target_archs.iter().for_each(|&arch| {
        let arch_str = arch.as_str();
        if arch_str.ends_with("sim") {
            simulator_archs.push(arch);
        } else if arch_str.starts_with(device_prefix) {
            device_archs.push(arch);
        } else {
            simulator_archs.push(arch);
        }
    });

    let mut grouped_archs = Vec::new();
    if !device_archs.is_empty() {
        grouped_archs.push(device_archs);
    }
    if !simulator_archs.is_empty() {
        grouped_archs.push(simulator_archs);
    }

    grouped_archs
}
