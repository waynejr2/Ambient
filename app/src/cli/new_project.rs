use std::path::Path;

use ambient_project::Identifier;
use anyhow::Context;
use convert_case::Casing;

pub(crate) fn new_project(project_path: &Path, name: Option<&str>) -> anyhow::Result<()> {
    let project_path = if let Some(name) = name { project_path.join(name) } else { project_path.to_owned() };
    let name = project_path.file_name().and_then(|s| s.to_str()).context("project path has no terminating segment")?;

    if project_path.is_dir() && std::fs::read_dir(&project_path)?.next().is_some() {
        anyhow::bail!("project path {project_path:?} is not empty");
    }

    let id = name.to_case(convert_case::Case::Snake);
    let id = Identifier::new(id).map_err(anyhow::Error::msg)?;

    let dot_cargo = project_path.join(".cargo");
    let src = project_path.join("src");
    std::fs::create_dir_all(&project_path).context("Failed to create project directory")?;
    std::fs::create_dir_all(&dot_cargo).context("Failed to create .cargo directory")?;
    std::fs::create_dir_all(&src).context("Failed to create src directory")?;

    std::fs::write(
        project_path.join("ambient.toml"),
        include_str!("new_project_template/ambient.toml").replace("{{id}}", id.as_ref()).replace("{{name}}", name),
    )
    .context("Failed to create ambient.toml")?;

    // Special-case creating an example in guest/rust/examples so that it "Just Works"
    enum ReplaceWith {
        PkgVersion,
        GuestRustRelative,
    }
    let replace_with = project_path
        .parent()
        .and_then(|parent_path| {
            let segments = parent_path.iter().collect::<Vec<_>>();
            let last_three = segments.iter().rev().take(3).rev().flat_map(|s| s.to_str()).collect::<Vec<_>>();
            if last_three == ["guest", "rust", "examples"] {
                Some(ReplaceWith::GuestRustRelative)
            } else {
                None
            }
        })
        .unwrap_or(ReplaceWith::PkgVersion);

    let replacement = match replace_with {
        ReplaceWith::PkgVersion => format!("ambient_api = \"{}\"", env!("CARGO_PKG_VERSION")),
        ReplaceWith::GuestRustRelative => r#"ambient_api = { path = "../../api" }"#.to_string(),
    };
    let template_cargo_toml = include_str!("new_project_template/Cargo.toml")
        .replace("{{id}}", id.as_ref())
        .replace("ambient_api = { path = \"../../../../guest/rust/api\" }", &replacement);

    std::fs::write(project_path.join("Cargo.toml"), template_cargo_toml).context("Failed to create Cargo.toml")?;

    std::fs::write(project_path.join(".gitignore"), include_str!("new_project_template/.gitignore"))
        .context("Failed to create .gitignore")?;

    std::fs::write(project_path.join("rust-toolchain.toml"), include_str!("new_project_template/rust-toolchain.toml"))
        .context("Failed to create rust-toolchain.toml")?;

    std::fs::write(dot_cargo.join("config.toml"), include_str!("new_project_template/.cargo/config.toml"))
        .context("Failed to create .cargo/config.toml")?;

    std::fs::write(src.join("lib.rs"), include_str!("new_project_template/src/lib.rs")).context("Failed to create src/lib.rs")?;

    log::info!("Project {name} with id {id} created at {project_path:?}");

    Ok(())
}
