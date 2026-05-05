use std::collections::HashMap;

use thiserror::Error;

// Include the generated component list from build.rs
include!(concat!(env!("OUT_DIR"), "/theme_components.rs"));

#[derive(Debug, Error)]
pub enum ThemeError {
    #[error("Missing theme asset: {0}")]
    MissingAsset(String),
    #[error("Failed to load template: {0}")]
    TemplateError(String),
    #[error("Invalid component: {0}")]
    InvalidComponent(String),
}

const THEME_TPL: &str = include_str!("../assets/theme.css.tpl");

pub struct ThemeAssembly {
    pub components: Vec<String>,
    pub user_style: Option<String>,
}

impl ThemeAssembly {
    pub fn bundle(&self) -> Result<String, ThemeError> {
        let mut component_map = HashMap::new();
        for &(filename, content) in THEME_COMPONENTS {
            component_map.insert(filename.to_string(), content);
        }

        let mut bundled_components = String::new();
        for comp in &self.components {
            if let Some(content) = component_map.get(comp) {
                bundled_components.push_str(content);
                if !content.ends_with('\n') {
                    bundled_components.push('\n');
                }
            } else {
                return Err(ThemeError::MissingAsset(comp.clone()));
            }
        }

        let mut final_css = THEME_TPL.to_string();

        // Inject components
        final_css = final_css.replace("/* {{COMPONENTS}} */", &bundled_components);

        // Inject user style
        let user_style_str = self.user_style.as_deref().unwrap_or("");
        final_css = final_css.replace("/* {{USER_STYLE}} */", user_style_str);

        Ok(final_css)
    }
}
