use thiserror::Error;

include!(concat!(env!("OUT_DIR"), "/theme_components.rs"));

const THEME_TPL: &str = include_str!("../assets/theme.css.tpl");

#[derive(Debug, Error)]
pub enum ThemeError {
    #[error("Missing theme asset: {0}")]
    MissingAsset(String),
    #[error("Template error: {0}")]
    TemplateError(String),
    #[error("Invalid component: {0}")]
    InvalidComponent(String),
}

pub struct ThemeAssembly {
    pub components: Vec<String>,
    pub user_style: Option<String>,
}

impl ThemeAssembly {
    pub fn new() -> Self {
        Self {
            components: COMPONENTS
                .iter()
                .map(|(name, _)| name.to_string())
                .collect(),
            user_style: None,
        }
    }

    pub fn bundle(&self) -> Result<String, ThemeError> {
        let mut components_css = String::new();

        for component in &self.components {
            let css = COMPONENTS
                .iter()
                .find(|(name, _)| name == component)
                .map(|(_, css)| css)
                .ok_or_else(|| ThemeError::MissingAsset(component.clone()))?;

            components_css.push_str(css);
            if !components_css.ends_with('\n') {
                components_css.push('\n');
            }
        }

        let mut bundled = THEME_TPL.replace("{{components}}", &components_css);

        let user_style_css = self.user_style.as_deref().unwrap_or("");
        bundled = bundled.replace("{{user_style}}", user_style_css);

        Ok(bundled)
    }
}

impl Default for ThemeAssembly {
    fn default() -> Self {
        Self::new()
    }
}
