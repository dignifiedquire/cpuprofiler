extern crate pkg_config;

/// Configures the crate to link against `lib_name`.
///
/// The library is first searched via the pkg-config file provided in
/// `pc_name`, which provides us accurate information on how to find the
/// library to link to. But because old gperftools did not supply such
/// files, this falls back to using the linker's path.
fn find_library(pc_name: &str, lib_name: &str) {
    match pkg_config::Config::new().atleast_version("2.0").probe(pc_name) {
        Ok(_) => (),
        Err(_) => println!("cargo:rustc-link-lib={}", lib_name),
    };
}

fn main () {
    find_library("libprofiler", "profiler");
    #[cfg(feature = "heap")] find_library("libtcmalloc", "tcmalloc");
}
