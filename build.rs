use std::env;
use std::process::Command;

fn main() {
  let out_dir = env::var("CARGO_MANIFEST_DIR").unwrap() + "/calc";

  // commands for build calc
  Command::new("make")
    .args(&["clean"])
    .current_dir(&out_dir)
    .status()
    .unwrap();
  Command::new("make").current_dir(&out_dir).status().unwrap();

  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-lib=static=calc");
}
