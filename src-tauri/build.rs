fn main() {
    println!("cargo:rerun-if-changed=config/api.toml");
    println!("cargo:rerun-if-changed=config/security.toml");
    // tauri-build only watches config files; watch icons too so icon changes rebuild the embedded dev icon
    println!("cargo:rerun-if-changed=icons/icon.icns");
    println!("cargo:rerun-if-changed=icons/icon.ico");
    println!("cargo:rerun-if-changed=icons/icon-windows.svg");
    println!("cargo:rerun-if-changed=icons/32x32.png");
    println!("cargo:rerun-if-changed=icons/128x128.png");
    println!("cargo:rerun-if-changed=icons/128x128@2x.png");
    tauri_build::build()
}
