use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::schema::{CanvasElement, GridPosition, Page, PageSection};

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RenderedPage {
  pub page_id: String,
  pub name: String,
  pub route: String,
  #[serde(default)]
  pub meta: crate::schema::PageMeta,
  #[serde(default)]
  pub layout_id: Option<String>,
  #[serde(default)]
  pub sections: HashMap<String, RenderedSection>,
  #[serde(default)]
  pub canvas_elements: Vec<RenderedElement>,
  #[serde(default)]
  #[ts(skip)]
  pub resolved_bindings: HashMap<String, serde_json::Value>,
}

impl RenderedPage {
  pub fn from_page(page: &Page, layout_id: Option<&str>) -> Self {
    Self {
      page_id: page.id.clone(),
      name: page.name.clone(),
      route: page.route.clone(),
      meta: page.meta.clone(),
      layout_id: layout_id.map(String::from),
      sections: page
        .sections
        .iter()
        .map(|(k, v)| (k.clone(), RenderedSection::from(v.clone())))
        .collect(),
      canvas_elements: page
        .canvas_elements
        .iter()
        .map(RenderedElement::from)
        .collect(),
      resolved_bindings: HashMap::new(),
    }
  }

  pub fn set_binding(&mut self, key: String, value: serde_json::Value) {
    self.resolved_bindings.insert(key, value);
  }

  pub fn get_binding(&self, key: &str) -> Option<&serde_json::Value> {
    self.resolved_bindings.get(key)
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RenderedSection {
  #[serde(default)]
  pub component_id: Option<String>,
  #[serde(default = "default_visible")]
  pub visible: bool,
  #[serde(default)]
  pub dynamic: bool,
}

fn default_visible() -> bool {
  true
}

impl From<PageSection> for RenderedSection {
  fn from(s: PageSection) -> Self {
    Self {
      component_id: s.component_id,
      visible: s.visible,
      dynamic: s.dynamic,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RenderedElement {
  pub id: String,
  pub component_id: String,
  #[serde(default)]
  pub grid_position: GridPosition,
  #[serde(default)]
  #[ts(skip)]
  pub props: HashMap<String, serde_json::Value>,
  #[serde(default)]
  #[ts(skip)]
  pub resolved_props: Option<serde_json::Value>,
  #[serde(default)]
  pub classes: String,
  #[serde(default)]
  pub children: Vec<String>,
  #[serde(default)]
  pub data_binding: Option<crate::schema::DataBinding>,
}

impl RenderedElement {
  pub fn from(element: &CanvasElement) -> Self {
    Self {
      id: element.id.clone(),
      component_id: element.component_id.clone(),
      grid_position: element.grid_position.clone(),
      props: element.props.clone(),
      resolved_props: None,
      classes: element.classes.clone(),
      children: element.children.clone(),
      data_binding: element.data_binding.clone(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RenderedCanvasElement {
  pub id: String,
  pub component_id: String,
  #[serde(default)]
  pub grid_position: GridPosition,
  #[serde(default)]
  #[ts(skip)]
  pub props: HashMap<String, serde_json::Value>,
  #[serde(default)]
  #[ts(skip)]
  pub resolved_props: Option<serde_json::Value>,
  #[serde(default)]
  pub classes: String,
  #[serde(default)]
  pub children: Vec<String>,
  #[serde(default)]
  pub data_binding: Option<crate::schema::DataBinding>,
}

impl From<&CanvasElement> for RenderedCanvasElement {
  fn from(e: &CanvasElement) -> Self {
    Self {
      id: e.id.clone(),
      component_id: e.component_id.clone(),
      grid_position: e.grid_position.clone(),
      props: e.props.clone(),
      resolved_props: None,
      classes: e.classes.clone(),
      children: e.children.clone(),
      data_binding: e.data_binding.clone(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ValidationResult {
  pub valid: bool,
  pub errors: Vec<ValidationError>,
}

impl ValidationResult {
  pub fn valid() -> Self {
    Self {
      valid: true,
      errors: vec![],
    }
  }

  pub fn invalid(errors: Vec<ValidationError>) -> Self {
    Self {
      valid: false,
      errors,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ValidationError {
  pub path: String,
  pub message: String,
  pub severity: String,
}

impl ValidationError {
  pub fn new(path: &str, message: &str, severity: &str) -> Self {
    Self {
      path: path.to_string(),
      message: message.to_string(),
      severity: severity.to_string(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct EventSignature {
  pub name: String,
  #[serde(default)]
  pub description: Option<String>,
  #[serde(default)]
  pub payload_type: Option<String>,
  #[serde(default)]
  pub return_type: Option<String>,
}

impl EventSignature {
  pub fn with_payload_type(mut self, payload_type: String) -> Self {
    self.payload_type = Some(payload_type);
    self
  }

  pub fn with_return_type(mut self, return_type: String) -> Self {
    self.return_type = Some(return_type);
    self
  }
}
