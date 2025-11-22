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

    writeln!(
        File::create("build-log.txt")?,
        "build log\nSTDOUT:\n{}\nSTDERR:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )?;

    Ok(())
}

#[inline]
fn add_assets_file(value: &str, file: &mut File, constant_name: &str, suffix: &str) {
    writeln!(
        file,
        "pub const {constant_name}_{suffix}: &str = r#\"{value}\"#;"
    )
    .unwrap();
}

fn generate_scripts_values(scripts: Vec<String>, module: bool) -> String {
    scripts
        .iter()
        .map(|script| {
            format!(
                r#"<script{} src="{script}"></script>"#,
                if module { r#" type="module""# } else { "" }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_styles_values(styles: Vec<String>) -> String {
    styles
        .iter()
        .map(|style| format!(r#"<link rel="stylesheet" href="{style}">"#))
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() -> std::io::Result<()> {
    #[cfg(not(debug_assertions))]
    {
        shell("mkdir static/assets/css")?;
        shell("pnpm i")?;
        shell("pnpm run build")?;
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let mut file = File::create(PathBuf::from(out_dir).join("generated_assets.rs"))?;

    #[cfg(not(debug_assertions))]
    {
        add_assets_file(
            &generate_styles_values(vec!["/assets/css/index.css".to_owned()]),
            &mut file,
            "ALL",
            "CSS",
        );
        add_assets_file(
            &generate_scripts_values(vec!["/assets/javascript/index.js".to_owned()], false),
            &mut file,
            "INDEX",
            "JS",
        );
        add_assets_file(
            &generate_scripts_values(vec!["/assets/javascript/code.js".to_owned()], false),
            &mut file,
            "CODE",
            "JS",
        );
    }

    #[cfg(debug_assertions)]
    {
        dotenvy::dotenv().ok();

        let vite_dev_url = std::env::var("VITE_DEV_URL").unwrap();
        add_assets_file(
            &format!(
                "{}\n    {}",
                generate_scripts_values(vec![format!("{vite_dev_url}/@vite/client"),], true),
                generate_styles_values(vec![format!("{vite_dev_url}/css/index.css"),])
            ),
            &mut file,
            "ALL",
            "CSS",
        );
        add_assets_file(
            &generate_scripts_values(vec![format!("{vite_dev_url}/ts/index.ts")], true),
            &mut file,
            "INDEX",
            "JS",
        );
        add_assets_file(
            &generate_scripts_values(vec![format!("{vite_dev_url}/ts/code.ts")], true),
            &mut file,
            "CODE",
            "JS",
        );
    }

    resource_dir("./static/assets/").build()
}
