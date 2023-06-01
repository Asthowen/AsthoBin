use static_files::resource_dir;
use std::fs::File;
use std::io::Write;
use std::process::Command;

#[allow(dead_code)]
fn shell(command: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(format!("Failed to run {cmd}", cmd = command).as_str());

    let mut file: File = File::create("build-log.txt").expect("Couldn't create file...");
    file.write(b"build log\n\n\n\nSTDOUT:\n")
        .expect("Couldn't write to build log");
    file.write_all(&output.stdout)
        .expect("Couldn't write to build log");
    file.write(b"\n\n\n\nSTDERR:\n")
        .expect("Couldn't write to build log");
    file.write_all(&output.stderr)
        .expect("Couldn't write to build log");
}

fn main() -> std::io::Result<()> {
    #[cfg(not(debug_assertions))]
    shell("mkdir static/assets/css");
    #[cfg(not(debug_assertions))]
    shell("pnpm i");
    #[cfg(not(debug_assertions))]
    shell("pnpm run prod");

    resource_dir("./static").build()
}
