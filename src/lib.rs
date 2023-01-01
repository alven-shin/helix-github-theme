use std::{collections::BTreeMap, error::Error};

use toml::{value::Map, Value};
use variant::{Palette, Variant};

mod variant;

const VARIANTS: [&str; 9] = [
    "dark",
    "dark_colorblind",
    "dark_dimmed",
    "dark_high_contrast",
    "dark_tritanopia",
    "light",
    "light_colorblind",
    "light_high_contrast",
    "light_tritanopia",
];

pub fn download_colorschemes() -> Result<Vec<Variant>, Box<dyn Error>> {
    let mut variants = Vec::new();

    for variant_name in VARIANTS {
        let response = ureq::get(&format!(
            "https://unpkg.com/@primer/primitives/dist/json/colors/{variant_name}.json"
        ))
        .call()?;
        let palette: Palette = response.into_json()?;
        variants.push(Variant {
            name: variant_name,
            palette,
        });
    }

    Ok(variants)
}

pub fn combine_tables(variant: &mut Value) {
    let x: Map<_, _> = variant
        .as_table_mut()
        .unwrap()
        .iter_mut()
        .map(|(category, table)| {
            // take ownership of table and rename all the values to include the category name
            let new_table = std::mem::replace(table, Value::Boolean(false))
                .try_into::<BTreeMap<String, Value>>()
                .unwrap();

            new_table.into_iter().map(|(k, v)| {
                let mut new_name = category.clone();
                new_name.push('.');
                new_name.push_str(&k);

                // scale colors, get all the individual colors
                if let Value::Array(table) = v {
                    new_name.push('.');
                    table
                        .iter()
                        .enumerate()
                        .map(|(i, color)| {
                            let mut new_name = new_name.clone();
                            new_name.push_str(&i.to_string());
                            (new_name, normalize_color(color))
                        })
                        .collect::<Vec<_>>()
                } else {
                    vec![(new_name, normalize_color(&v))]
                }
            })
        })
        .into_iter()
        .flatten()
        .flatten()
        .collect();
    *variant = x.into();
}

fn normalize_color(color_str: &Value) -> Value {
    Value::from(
        csscolorparser::parse(color_str.as_str().unwrap())
            .unwrap()
            .to_hex_string(),
    )
}
