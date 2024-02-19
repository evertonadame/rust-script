
use serde::{Deserialize, Serialize};
use crate::{utils::empty_string, utils::empty_json};

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplatesResponse {
    pub data: Data,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub templates: Vec<Template>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub version: Option<String>,
    pub component_type: Option<String>,
    pub render_type: Option<String>,
    pub touchpoint: Option<Vec<String>>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub schema: Schema,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub json: Option<serde_json::Value>,
    pub ui: Option<serde_json::Value>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateFile {
    pub name: String,
    pub content: Schema,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateWithoutId {
    pub name: Option<String>,
    pub description: Option<String>,
    pub active: Option<bool>,
    pub version: Option<String>,
    pub component_type: Option<String>,
    pub render_type: Option<String>,
    pub touchpoint: Option<Vec<String>>,
    pub image: Option<String>,
    pub thumbnail: Option<String>,
    pub schema: Schema,
}

// Implementação para converter de Template para TemplateWithoutId

impl From<Template> for TemplateWithoutId {
    fn from(template: Template) -> Self {
        TemplateWithoutId {
            name: Some(template.name.clone().unwrap_or_else(empty_string)),
            description: Some(template.description.clone().unwrap_or_else(empty_string)),
            active: Some(template.active.unwrap_or_else(|| true)),
            version: Some(template.version.clone().unwrap_or_else(empty_string)),
            component_type: Some(template.component_type.clone().unwrap_or_else(empty_string)),
            render_type: Some(template.render_type.clone().unwrap_or_else(empty_string)),
            touchpoint: Some(template.touchpoint.clone().unwrap_or_else(|| vec![])),
            image: template.thumbnail.clone(),
            thumbnail: template.thumbnail.clone(),
            schema: Schema {
                json: Some(template.schema.json.clone().unwrap_or_else(empty_json)),
                ui: Some(template.schema.ui.clone().unwrap_or_else(empty_json)),
            },
        }
    }
}
