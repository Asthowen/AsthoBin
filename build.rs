use static_files::resource_dir;
#[cfg(not(debug_assertions))]
use {std::fs::File, std::io::Write, std::process::Command};

#[cfg(not(debug_assertions))]
fn shell(command: &str) -> std::io::Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .unwrap_or_else(|_| panic!("Failed to run {command}"));

    let mut file = File::create("build-log.txt")?;
    writeln!(
        file,
        "build log\nSTDOUT:\n{}\nSTDERR:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    #[cfg(not(debug_assertions))]
    {
        shell("mkdir static/assets/css")?;
        shell("pnpm i")?;
        shell("pnpm run prod")?;
    }

    resource_dir("./static/assets/").build()
}
