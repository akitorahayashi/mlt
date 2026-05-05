// Include the generated component list from build.rs
include!(concat!(env!("OUT_DIR"), "/theme_components.rs"));

pub fn bundle(user_style: Option<&str>) -> String {
    let mut css = String::new();
    css.push_str("/* @theme mlt-default */\n");
    css.push_str("@import 'default';\n");

    for &(_, content) in THEME_COMPONENTS {
        css.push('\n');
        css.push_str(content);
        if !content.ends_with('\n') {
            css.push('\n');
        }
    }

    if let Some(style) = user_style {
        if !style.is_empty() {
            css.push('\n');
            css.push_str(style);
            if !style.ends_with('\n') {
                css.push('\n');
            }
        }
    }

    css
}
