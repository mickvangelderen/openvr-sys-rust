extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    build_openvr();
    generate_bindings();
}

const OPENVR_PATH: &str = "openvr";
const WRAPPER_PATH: &str = "wrapper.hpp";

fn build_openvr() {
    println!("cargo:rerun-if-changed={}", OPENVR_PATH);
    println!("cargo:rerun-if-changed={}", WRAPPER_PATH);

    let target_os = TargetOs::from_str(&env::var("CARGO_CFG_TARGET_OS").unwrap());
    let target_pointer_width =
        TargetPointerWidth::from_str(&env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap());

    let dst = {
        let mut config = cmake::Config::new(OPENVR_PATH);

        match target_os {
            TargetOs::Linux => {}
            TargetOs::Macos => {
                config.define("BUILD_UNIVERSAL", "OFF");
            }
            TargetOs::Windows => {
                // Build errors on #warning statements without this.
                config.cxxflag("/DWIN32");
            }
        }

        config.build()
    };

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display(),
    );

    println!(
        "cargo:rustc-link-lib={}={}",
        match target_os {
            TargetOs::Linux | TargetOs::Windows => "static",
            TargetOs::Macos => "framework",
        },
        match target_os {
            TargetOs::Linux | TargetOs::Macos => "openvr_api",
            TargetOs::Windows => match target_pointer_width {
                TargetPointerWidth::W32 => "openvr_api",
                TargetPointerWidth::W64 => "openvr_api64",
            },
        }
    );

    match target_os {
        TargetOs::Linux => println!("cargo:rustc-link-lib=stdc++"),
        TargetOs::Macos => println!("cargo:rustc-link-lib=c++"),
        TargetOs::Windows => println!("cargo:rustc-link-lib=shell32"),
    }
}

fn generate_bindings() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(WRAPPER_PATH)
        // Blacklist these deprecated types.
        .blacklist_item("HmdError")
        .blacklist_item("Hmd_Eye")
        .blacklist_item("ColorSpace")
        .blacklist_item("HmdTrackingResult")
        .blacklist_item("TrackedDeviceClass")
        .blacklist_item("TrackingUniverseOrigin")
        .blacklist_item("TrackedDeviceProperty")
        .blacklist_item("TrackedPropertyError")
        .blacklist_item("VRSubmitFlags_t")
        .blacklist_item("VRState_t")
        .blacklist_item("CollisionBoundsStyle_t")
        .blacklist_item("VROverlayError")
        .blacklist_item("VRFirmwareError")
        .blacklist_item("VRCompositorError")
        .blacklist_item("VRScreenshotsError")
        // Provide definitions for these types in src/manual.rs.
        .blacklist_type("VREvent_t")
        .blacklist_type("VRControllerState001_t")
        .blacklist_type("RenderModel_TextureMap_t")
        .blacklist_type("RenderModel_t")
        // Some more generation options.
        .generate_comments(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        // Nightly is cool.
        .rust_target(bindgen::RustTarget::Nightly)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

enum TargetOs {
    Linux,
    Macos,
    Windows,
}

impl TargetOs {
    fn from_str(s: &str) -> Self {
        match s {
            "linux" => Self::Linux,
            "macos" => Self::Macos,
            "windows" => Self::Windows,
            _ => unimplemented!(),
        }
    }
}

enum TargetPointerWidth {
    W32,
    W64,
}

impl TargetPointerWidth {
    fn from_str(s: &str) -> Self {
        match s {
            "32" => Self::W32,
            "64" => Self::W64,
            _ => unimplemented!(),
        }
    }
}
