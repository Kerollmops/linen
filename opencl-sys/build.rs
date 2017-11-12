extern crate bindgen;

use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rustc-link-search=framework=/System/Library/Frameworks/");
    println!("cargo:rustc-link-lib=framework=OpenCL");

    let headers = env::var("CARGO_MANIFEST_DIR").unwrap() + "/opencl22/CL/";
    let headers = Path::new(&headers);

    let bindings = bindgen::Builder::default()
        .header(headers.join("cl.h").to_string_lossy())
        .header(headers.join("cl_d3d10.h").to_string_lossy())
        .header(headers.join("cl_d3d11.h").to_string_lossy())
        .header(headers.join("cl_dx9_media_sharing.h").to_string_lossy())
        .header(headers.join("cl_dx9_media_sharing_intel.h").to_string_lossy())
        .header(headers.join("cl_egl.h").to_string_lossy())
        .header(headers.join("cl_ext.h").to_string_lossy())
        .header(headers.join("cl_ext_intel.h").to_string_lossy())
        .header(headers.join("cl_gl.h").to_string_lossy())
        .header(headers.join("cl_gl_ext.h").to_string_lossy())
        .header(headers.join("cl_platform.h").to_string_lossy())
        .header(headers.join("cl_va_api_media_sharing_intel.h").to_string_lossy())
        .header(headers.join("opencl.h").to_string_lossy())
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
