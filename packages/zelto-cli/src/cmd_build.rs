use anyhow::Result;
use std::process::Command;
use std::path::Path;

pub fn run(path: &str, target: &str, release: bool) -> Result<()> {
    println!("Building Zelto app at: {path}");

    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    cmd.args(["--target", target]);
    if release {
        cmd.arg("--release");
    }
    cmd.current_dir(Path::new(path));

    let status = cmd.status()?;
    if !status.success() {
        anyhow::bail!("Build failed");
    }

    let profile = if release { "release" } else { "debug" };
    println!("Build complete: target/{target}/{profile}/");

    Ok(())
}
