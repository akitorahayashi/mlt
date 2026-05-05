use thiserror::Error;

include!(concat!(env!("OUT_DIR"), "/components.rs"));

#[derive(Debug, Default)]
pub struct ThemeAssembly {
    pub components: Vec<String>,
    pub user_style: Option<String>,
}

#[derive(Debug, Error)]
pub enum ThemeError {
    #[error("Missing theme asset: {0}")]
    MissingAsset(String),
    #[error("Template error: {0}")]
    TemplateError(String),
    #[error("Invalid component: {0}")]
    InvalidComponent(String),
}

impl ThemeAssembly {
    pub fn bundle(&self) -> Result<String, ThemeError> {


        let template = include_str!("../assets/theme.css.tpl");

        // Find the components to bundle
        let mut resolved_components = String::new();
        for req_component in &self.components {
            let found = COMPONENTS.iter().find(|(name, _)| name == req_component);
            if let Some((_, content)) = found {
                resolved_components.push_str(content);
                if !resolved_components.ends_with('\n') {
                    resolved_components.push('\n');
                }
            } else {
                return Err(ThemeError::MissingAsset(req_component.clone()));
            }
        }

        let user_style = self.user_style.as_deref().unwrap_or("");

        let bundled = template
            .replace("{{components}}", &resolved_components)
            .replace("{{user_style}}", user_style);

        Ok(bundled)
    }
}
