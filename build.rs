extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn generate_bindings() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
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

fn link_openvr() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_os = TargetOs::from_str(&env::var("CARGO_CFG_TARGET_OS").unwrap());
    let target_pointer_width =
        TargetPointerWidth::from_str(&env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap());

    println!(
        "cargo:rustc-link-search=native={}",
        [
            manifest_dir.as_path(),
            "openvr".as_ref(),
            match target_os {
                TargetOs::Linux => "bin",
                TargetOs::Macos | TargetOs::Windows => "lib",
            }
            .as_ref(),
            format!(
                "{}{}",
                match target_os {
                    TargetOs::Linux => "linux",
                    TargetOs::Macos => "macos",
                    TargetOs::Windows => "win",
                },
                match target_pointer_width {
                    TargetPointerWidth::W32 => "32",
                    TargetPointerWidth::W64 => "64",
                }
            )
            .as_ref()
        ]
        .iter()
        .collect::<PathBuf>()
        .display()
    );

    match target_os {
        TargetOs::Linux | TargetOs::Windows => println!("cargo:rustc-link-lib=static=openvr_api"),
        TargetOs::Macos => println!("cargo:rustc-link-lib=framework=openvr_api"),
    }

    match target_os {
        TargetOs::Linux => println!("cargo:rustc-link-lib=stdc++"),
        TargetOs::Macos => println!("cargo:rustc-link-lib=c++"),
        TargetOs::Windows => println!("cargo:rustc-link-lib=shell32"),
    }
}

fn main() {
    link_openvr();
    generate_bindings();
}
