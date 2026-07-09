use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::schema::{Page, RenderedPage, UiSchema};
use crate::AppError;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RouteConfig {
  pub page_id: String,
  #[serde(default)]
  pub layout_id: Option<String>,
  #[serde(default)]
  pub guards: Vec<String>,
  #[serde(default)]
  pub middleware: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RouteMatch {
  pub page: Page,
  #[serde(default)]
  pub params: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SchemaRouter {
  routes: HashMap<String, RouteConfig>,
  schema: UiSchema,
}

impl SchemaRouter {
  pub fn from_schema(schema: &UiSchema) -> Self {
    Self {
      routes: HashMap::new(),
      schema: schema.clone(),
    }
  }

  pub fn with_routes(schema: &UiSchema, routes: HashMap<String, RouteConfig>) -> Self {
    Self {
      routes,
      schema: schema.clone(),
    }
  }

  pub fn add_route(&mut self, path: String, config: RouteConfig) {
    self.routes.insert(path, config);
  }

  pub fn remove_route(&mut self, path: &str) {
    self.routes.remove(path);
  }

  pub fn get_route(&self, path: &str) -> Option<&RouteConfig> {
    self.routes.get(path)
  }

  pub fn routes(&self) -> impl Iterator<Item = (&String, &RouteConfig)> {
    self.routes.iter()
  }

  pub fn match_route(&self, path: &str) -> Option<RouteMatch> {
    let config = self.routes.get(path)?;

    let page = self.schema.pages.iter().find(|p| p.id == config.page_id)?;

    Some(RouteMatch {
      page: page.clone(),
      params: HashMap::new(),
    })
  }

  pub fn resolve(&self, path: &str) -> Result<RenderedPage> {
    let config = self
      .routes
      .get(path)
      .ok_or_else(|| AppError::NotFound(format!("Route '{}' not found", path)))?;

    let page = self
      .schema
      .pages
      .iter()
      .find(|p| p.id == config.page_id)
      .ok_or_else(|| AppError::NotFound(format!("Page '{}'", config.page_id)))?;

    let rendered = RenderedPage::from_page(page, config.layout_id.as_deref());
    Ok(rendered)
  }

  pub fn has_route(&self, path: &str) -> bool {
    self.routes.contains_key(path)
  }

  pub fn route_count(&self) -> usize {
    self.routes.len()
  }
}
