use std::fmt::Write;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize)]
struct UiLang {
    file: String,
    lang_id: String,
}

#[derive(Deserialize)]
struct LangInfo {
    ui: Vec<UiLang>,
}

pub fn main() {
    let lang_info: LangInfo = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/lang/lang.json"
    )))
    .unwrap();

    let file_count = lang_info.ui.len();
    let lang_dir = format!("{}/lang", env!("CARGO_MANIFEST_DIR"));
    let files = lang_info
        .ui
        .into_iter()
        .fold(String::new(), |mut output, l| {
            let _ = write!(
                output,
                r#" (langid!("{}"), include_str!("{lang_dir}/{}.ftl")), "#,
                l.lang_id, l.file
            );
            output
        });

    let contents = format!(
        r#"
    use unic_langid::{{langid, LanguageIdentifier}};

    pub const LANGUAGE_FILES: [(LanguageIdentifier, &str); {}] = [{}];    
    "#,
        file_count, files
    );

    std::fs::write(
        Path::new(env!("CARGO_TARGET_DIR")).join("lang_in.rs"),
        contents,
    )
    .unwrap();
}
