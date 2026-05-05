use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("theme_components.rs");

    let css_dir = Path::new("src/assets/css");

    // Ensure the build script is re-run if the css directory changes
    println!("cargo:rerun-if-changed=src/assets/css");

    let mut components = Vec::new();

    if css_dir.exists() && css_dir.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(css_dir)
            .unwrap()
            .map(|res| res.unwrap())
            .collect();

        // Sort entries by name to ensure consistent ordering
        entries.sort_by_key(|dir| dir.path());

        for entry in entries {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "css") {
                let filename = path.file_name().unwrap().to_str().unwrap().to_string();
                let relative_path = path
                    .strip_prefix("src/assets/css")
                    .unwrap()
                    .display()
                    .to_string();
                // Replace backslashes on windows just in case, though strip_prefix from forward-slash path usually is fine.
                let relative_path = relative_path.replace("\\", "/");
                components.push(format!("    (\"{}\", include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/assets/css/{}\"))),", filename, relative_path));
            }
        }
    }

    let generated_code = format!(
        "pub const THEME_COMPONENTS: &[(&str, &str)] = &[\n{}\n];\n",
        components.join("\n")
    );

    fs::write(&dest_path, generated_code).unwrap();
}
