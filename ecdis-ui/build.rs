fn main() -> Result<(), Box<dyn std::error::Error>> {
    slint_build::compile("ui/app.slint")?;
    println!(
        "cargo:rustc-env=ECDIS_UI_VERSION={}",
        env!("CARGO_PKG_VERSION")
    );
    // Workspace-pinned Slint; keep in sync with [workspace.dependencies] in root Cargo.toml.
    println!("cargo:rustc-env=SLINT_VERSION=1.16.1");
    if let Ok(rev) = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        && rev.status.success()
    {
        let hash = String::from_utf8_lossy(&rev.stdout).trim().to_string();
        if !hash.is_empty() {
            println!("cargo:rustc-env=ECDIS_UI_GIT_REV={hash}");
        }
    }
    Ok(())
}
