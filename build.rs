use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=ping-view/build");
    println!("cargo:rerun-if-changed=ping-view/src");

    let web_build = Path::new("ping-view/build");
    if !web_build.exists() {
        let result = Command::new("npm")
            .args(&["run-script", "build"])
            .current_dir("ping-view")
            .spawn()
            .expect("Frontend build error!")
            .wait()
            .unwrap();
        assert!(result.success());
    }
}
