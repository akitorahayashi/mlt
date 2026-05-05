use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("theme_components.rs");

    let css_dir = Path::new("src/assets/css");
    let mut components = Vec::new();

    if css_dir.exists() && css_dir.is_dir() {
        let mut entries = fs::read_dir(css_dir)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        entries.sort();

        for path in entries {
            if path.is_file() && path.extension().unwrap_or_default() == "css" {
                let file_name = path.file_name().unwrap().to_str().unwrap();
                // Make the path relative to the OUT_DIR or use absolute path
                // For simplicity we will use absolute path here with include_str!
                // Wait, include_str! is relative to the file where it's used.
                // It's used in OUT_DIR, so we need to point it to the actual file.
                // Let's just generate the content of the file or use an absolute path.
                let absolute_path = fs::canonicalize(&path).unwrap();
                let path_str = absolute_path.to_str().unwrap().replace("\\", "\\\\");

                components.push(format!("(\"{}\", include_str!(\"{}\"))", file_name, path_str));
            }
        }
    }

    let code = format!(
        "pub const COMPONENTS: &[(&str, &str)] = &[\n    {}\n];\n",
        components.join(",\n    ")
    );

    fs::write(&dest_path, code).unwrap();
    println!("cargo:rerun-if-changed=src/assets/css");
}
