extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::path::Path;

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", Path::new(&dir).join("openvr/lib/linux64").display());
    println!("cargo:rustc-link-lib=openvr_api");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Force c++
        .clang_arg("-x")
        .clang_arg("c++")
        // Set std
        .clang_arg("-std=c++11")
        // Only emit bingings for these types (which should be all we need for OpenVR).
        .whitelist_type("vr::.*")
        .whitelist_function("vr::.*")
        .whitelist_var("vr::.*")
        // Blacklist these deprecated types.
        .blacklist_item("vr::Hmd_Error")
        .blacklist_item("vr::Hmd_Eye")
        .blacklist_item("vr::ColorSpace")
        .blacklist_item("vr::HmdTrackingResult")
        .blacklist_item("vr::TrackedDeviceClass")
        .blacklist_item("vr::TrackingUniverseOrigin")
        .blacklist_item("vr::TrackedDeviceProperty")
        .blacklist_item("vr::TrackedPropertyError")
        .blacklist_item("vr::VRSubmitFlags_t")
        .blacklist_item("vr::VRState_t")
        .blacklist_item("vr::CollisionBoundsStyle_t")
        .blacklist_item("vr::VROverlayError")
        .blacklist_item("vr::VRFirmwareError")
        .blacklist_item("vr::VRCompositorError")
        .blacklist_item("vr::VRScreenshotsError")
        // Forget about this inline crap.
        .opaque_type("vr::IVRSettingsHelper")
        // Forget this crap, think its because stdint is imported?.
        .opaque_type("std::.*")
        // Some more generation options.
        .generate_comments(true)
        .layout_tests(false)
        .prepend_enum_name(false)
        .enable_cxx_namespaces()
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
