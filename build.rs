use static_files::resource_dir;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
#[cfg(not(debug_assertions))]
use std::process::Command;

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

#[inline]
fn add_assets_script_file(value: &str, file: &mut File, constant_name: &str) {
    writeln!(file, "pub const {constant_name}_JS: &str = r#\"{value}\"#;").unwrap();
}

fn generate_scripts_values(scripts: Vec<String>) -> String {
    scripts
        .iter()
        .map(|script| format!(r#"<script type="module" src="{script}"></script>"#))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    #[cfg(not(debug_assertions))]
    {
        shell("mkdir static/assets/css")?;
        shell("pnpm i")?;
        shell("pnpm run prod")?;
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut file = File::create(PathBuf::from(out_dir).join("generated_assets.rs"))?;

    #[cfg(not(debug_assertions))]
    {
        add_assets_script_file(
            &generate_scripts_values(vec!["/assets/javascript/index.js".to_owned()]),
            &mut file,
            "INDEX",
        );
        add_assets_script_file(
            &generate_scripts_values(vec!["/assets/javascript/code.js".to_owned()]),
            &mut file,
            "CODE",
        );
    }

    #[cfg(debug_assertions)]
    {
        let vite_dev_url = std::env::var("VITE_DEV_URL").unwrap();
        add_assets_script_file(
            &generate_scripts_values(vec![
                format!("{vite_dev_url}/@vite/client"),
                format!("{vite_dev_url}/index.ts"),
            ]),
            &mut file,
            "INDEX",
        );
        add_assets_script_file(
            &generate_scripts_values(vec![
                format!("{vite_dev_url}/@vite/client"),
                format!("{vite_dev_url}/code.ts"),
            ]),
            &mut file,
            "CODE",
        );
    }

    resource_dir("./static/assets/").build()
}
