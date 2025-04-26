use std::{env, fs};
use std::path::{Path, PathBuf};

const CARGO_MANIFEST_DIR: &str = std::env!("CARGO_MANIFEST_DIR");
const FONT: &str = std::env!("FONT");

fn main() {
    embuild::espidf::sysenv::output();

    let out_dir = env::var("OUT_DIR").unwrap();

    let dest_path = Path::new(&out_dir).join("font_bytes.rs");

    let mut generated = String::new();
    generated.push_str("pub static FONT_BYTES: [&'static [u8]; 10] = [\n");

    for i in 0..=9 {
        let path = PathBuf::from(&CARGO_MANIFEST_DIR)
            .join(format!("src/Fonts/{}/{}.bmp", FONT, i))
            .display()
            .to_string();

        generated.push_str(&format!(
            "    include_bytes!(r#\"{}\"#),\n",
            path
        ));
    }

    generated.push_str("];\n");

    fs::write(&dest_path, generated).unwrap();
    println!("cargo:warning=Wrote font import file to: {}", dest_path.to_str().unwrap());
}
