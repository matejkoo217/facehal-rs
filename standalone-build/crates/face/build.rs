fn main() {
    // The standalone `iface_client` probe binary links against the NDK Binder.
    // Prefer a device-pulled syslib dir (FACEHAL_PROBE_SYSLIBS) because the NDK
    // API-29 stub libbinder_ndk.so lacks AServiceManager_getService.
    if let Ok(dir) = std::env::var("FACEHAL_PROBE_SYSLIBS") {
        println!("cargo:rustc-link-search=native={dir}");
    } else if let Ok(lib) = std::env::var("BINDER_NDK_LIB_DIR") {
        println!("cargo:rustc-link-search=native={lib}");
    }
    println!("cargo:rustc-link-lib=dylib=binder_ndk");
    println!("cargo:rustc-link-lib=dylib=nativewindow");
}
