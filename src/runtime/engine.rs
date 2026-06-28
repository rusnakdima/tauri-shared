use std::collections::HashMap;
use std::sync::Arc;

use serde_json::Value;

use crate::schema::{ComponentDef, Layout, Page, Theme, UiSchema};
use crate::storage::SignalStore;
use crate::AppError;
use crate::Result;

pub type Callable = Arc<dyn Fn(Vec<Value>) -> Result<Value> + Send + Sync>;

pub type GuardFn = Arc<dyn Fn(&str) -> Result<bool> + Send + Sync>;

pub type MiddlewareFn = Arc<dyn Fn(&mut crate::schema::RenderedPage) -> Result<()> + Send + Sync>;

#[derive(Clone)]
pub struct SduiEngine {
    pub schema: UiSchema,
    pub theme: Theme,
    pub store: Arc<SignalStore>,
    pub functions: HashMap<String, Callable>,
}

impl SduiEngine {
    pub fn new(schema: UiSchema, theme: Theme) -> Self {
        Self {
            schema,
            theme,
            store: Arc::new(SignalStore::new()),
            functions: HashMap::new(),
        }
    }

    pub fn with_store(schema: UiSchema, theme: Theme, store: Arc<SignalStore>) -> Self {
        Self {
            schema,
            theme,
            store,
            functions: HashMap::new(),
        }
    }

    pub fn load_schema(&mut self, schema: UiSchema) {
        self.schema = schema;
    }

    pub fn schema(&self) -> &UiSchema {
        &self.schema
    }

    pub fn theme(&self) -> &Theme {
        &self.theme
    }

    pub fn apply_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }

    pub fn get_page(&self, page_id: &str) -> Option<&Page> {
        self.schema.pages.iter().find(|p| p.id == page_id)
    }

    pub fn get_layout(&self, layout_id: &str) -> Option<&Layout> {
        self.schema.layouts.iter().find(|l| l.id == layout_id)
    }

    pub fn get_component(&self, component_id: &str) -> Option<&ComponentDef> {
        self.schema
            .components
            .iter()
            .chain(self.schema.shared_components.iter())
            .find(|c| c.id == component_id)
    }

    pub fn register_function<F>(&mut self, name: String, callable: F)
    where
        F: Fn(Vec<Value>) -> Result<Value> + Send + Sync + 'static,
    {
        self.functions.insert(name, Arc::new(callable));
    }

    pub fn call_function(&self, name: &str, args: Vec<Value>) -> Result<Value> {
        self.functions
            .get(name)
            .ok_or_else(|| AppError::NotFound(format!("Function '{}' not found", name)))?(
            args
        )
    }

    pub fn has_function(&self, name: &str) -> bool {
        self.functions.contains_key(name)
    }

    pub fn get_state(&self, key: &str) -> Option<Value> {
        self.store.get(key)
    }

    pub fn set_state(&self, key: &str, value: Value) {
        self.store.set(key, value);
    }

    pub fn update_state<F>(&self, key: &str, f: F)
    where
        F: FnOnce(&Value) -> Value,
    {
        self.store.update(key, f);
    }

    pub fn delete_state(&self, key: &str) {
        self.store.delete(key);
    }

    pub fn subscribe_to_state<F>(&self, callback: F)
    where
        F: Fn(&str, &Value) + Send + Sync + 'static,
    {
        self.store.subscribe(callback);
    }

    pub fn export_state(&self) -> Value {
        self.store.to_json()
    }

    pub fn import_state(&self, json: Value) {
        self.store.from_json(json);
    }

    pub fn state_keys(&self) -> Vec<String> {
        self.store.keys()
    }

    pub fn resolve_binding(&self, binding: &crate::schema::DataBinding) -> Result<Value> {
        let entity_data = self.store.get(&binding.entity).ok_or_else(|| {
            AppError::NotFound(format!("Entity '{}' not found in store", binding.entity))
        })?;

        if let Some(field) = &binding.field {
            entity_data.get(field).cloned().ok_or_else(|| {
                AppError::NotFound(format!("Field '{}' on entity '{}'", field, binding.entity))
            })
        } else {
            Ok(entity_data)
        }
    }

    pub fn render_page(&self, route: &str) -> Result<crate::schema::RenderedPage> {
        let page = self
            .get_page(route)
            .ok_or_else(|| AppError::NotFound(format!("Page for route '{}' not found", route)))?;

        let mut rendered = crate::schema::RenderedPage::from_page(page, None);

        for element in &mut rendered.canvas_elements {
            if let Some(ref binding) = element.data_binding {
                if let Ok(value) = self.resolve_binding(binding) {
                    element.resolved_props = Some(value);
                }
            }
        }

        Ok(rendered)
    }

    pub fn validate(&self) -> crate::schema::ValidationResult {
        crate::runtime::validation::validate_schema(&self.schema)
    }

    pub fn dispatch_event(&self, element_id: &str, event: &str, payload: Value) -> Result<()> {
        let handler_name = format!("on_{}_{}", element_id, event);
        if self.has_function(&handler_name) {
            self.call_function(&handler_name, vec![payload])?;
        }
        Ok(())
    }
}

impl From<UiSchema> for SduiEngine {
    fn from(schema: UiSchema) -> Self {
        let theme = schema
            .app
            .settings
            .themes
            .first()
            .cloned()
            .unwrap_or_else(crate::schema::theme::get_light_theme);
        Self::new(schema, theme)
    }
}
