use anyhow::Result;

pub fn run(path: &str) -> Result<()> {
    println!("Starting Zelto dev server at: {path}");
    println!("Hot reload: watching *.zelto files...");

    // Phase 3: integrate notify crate watcher + IR compiler + runtime reload
    anyhow::bail!("Dev mode with hot reload coming in Phase 3. Use `cargo run` for now.")
}
