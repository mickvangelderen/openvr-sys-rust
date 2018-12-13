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
        .raw_line("use super::manual::*;")
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
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    // NOTE(mickvangelderen): The improper way to do this so we get
    // racer completion and easier to inspect the generated src.
    let out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn build_openvr() {
    let dst = cmake::Config::new("openvr").build();
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );

    #[cfg(all(windows, target_pointer_width = "64"))]
    println!("cargo:rustc-link-lib=static=openvr_api64");

    #[cfg(not(all(windows, target_pointer_width = "64")))]
    println!("cargo:rustc-link-lib=static=openvr_api");

    #[cfg(target_os="linux")]
    println!("cargo:rustc-link-lib=stdc++");

    #[cfg(target_os="macos")]
    println!("cargo:rustc-link-lib=c++");
}

fn main() {
    build_openvr();
    generate_bindings();
}
