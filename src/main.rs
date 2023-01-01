use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use helix_github_theme::{combine_tables, download_colorschemes};
use toml::Value;

fn main() {
    let mut light_template = fs::read_to_string("light_template.toml").unwrap();
    light_template.push('\n');
    let mut dark_template = fs::read_to_string("dark_template.toml").unwrap();
    dark_template.push('\n');

    let variants = download_colorschemes().unwrap();
    let mut path = PathBuf::new();
    path.push("dist");
    fs::create_dir(&path).ok();

    for mut variant in variants {
        let variant_name = std::mem::take(&mut variant.name);

        let mut toml = Value::try_from(variant).unwrap();
        combine_tables(toml.as_table_mut().unwrap().get_mut("palette").unwrap());
        let toml = toml::to_string(&toml).unwrap();

        path.push(format!("github_{}.toml", variant_name));
        let mut file = File::create(&path).unwrap();

        // write template
        if variant_name.contains("light") {
            file.write_all(light_template.as_bytes()).unwrap();
        } else {
            file.write_all(dark_template.as_bytes()).unwrap();
        }

        // write palette
        file.write_all(toml.as_bytes()).unwrap();

        path.pop();
    }
}
