use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let device_name = "esp32c6";
    // Define all necessary configuration symbols for the configured device:
    println!("cargo:rustc-cfg={}", device_name);

    // Put the linker script somewhere the linker can find it:
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    fs::copy("ld/link-lp.x", out.join("link.x"))?;
    println!("cargo:rerun-if-changed=ld/link-lp.x");

    Ok(())
}
