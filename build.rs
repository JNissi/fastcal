use std::process::Command;

fn main() {
    Command::new("glib-compile-resources")
        .arg("--target=src/fastcal.gresource")
        .args(&["--sourcedir=src", "src/fastcal.xml"])
        .status()
        .unwrap();

    // List all files that go into the resources file for rerun check.
    println!("cargo:rerun-if-changed=src/fastcal.xml");
    println!("cargo:rerun-if-changed=src/fastcal.glade");
}