use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("components.rs");

    let css_dir = Path::new("src/assets/css");
    let mut components = Vec::new();

    let mut entries: Vec<_> = fs::read_dir(css_dir)
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("css") {
            let name = path.file_name().unwrap().to_str().unwrap();
            let content = fs::read_to_string(&path).unwrap();
            components.push(format!("(\"{}\", r###\"{}\"###)", name, content));
        }
    }

    let generated_code = format!(
        "pub const COMPONENTS: &[(&str, &str)] = &[\n    {},\n];\n",
        components.join(",\n    ")
    );

    fs::write(&dest_path, generated_code).unwrap();
    println!("cargo:rerun-if-changed=src/assets/css");
    println!("cargo:rerun-if-changed=build.rs");
}
