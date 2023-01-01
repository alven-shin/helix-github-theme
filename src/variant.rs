macro_rules! palette_struct {
    ($($field:ident),* $(,)?) => {
        #[derive(Debug, serde::Deserialize, serde::Serialize)]
        pub struct Palette {
            $($field: ureq::serde_json::Value),*
        }
    };
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Variant {
    #[serde(skip)]
    pub name: &'static str,
    pub palette: Palette,
}

palette_struct! {
    fg,
    accent,
    canvas,
    border,
    // shadow,
    neutral,
    success,
    attention,
    severe,
    danger,
    open,
    closed,
    done,
    sponsors,
    scale,
}
