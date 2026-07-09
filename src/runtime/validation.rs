use std::collections::HashSet;

use crate::schema::{
  CanvasElement, ComponentDef, Page, UiSchema, ValidationError, ValidationResult,
};

pub fn validate_schema(schema: &UiSchema) -> ValidationResult {
  let mut errors: Vec<ValidationError> = Vec::new();
  let mut seen_ids: HashSet<&str> = HashSet::new();

  for page in &schema.pages {
    let page_errors = validate_page(page, schema);
    errors.extend(page_errors.errors);
    if !seen_ids.insert(page.id.as_str()) {
      errors.push(ValidationError::new(
        &format!("pages[{}]", page.id),
        &format!("Duplicate page ID: '{}'", page.id),
        "duplicate",
      ));
    }
  }

  for layout in &schema.layouts {
    if !seen_ids.insert(layout.id.as_str()) {
      errors.push(ValidationError::new(
        &format!("layouts[{}]", layout.id),
        &format!("Duplicate layout ID: '{}'", layout.id),
        "duplicate",
      ));
    }
  }

  for component in schema
    .components
    .iter()
    .chain(schema.shared_components.iter())
  {
    if !seen_ids.insert(component.id.as_str()) {
      errors.push(ValidationError::new(
        &format!("components[{}]", component.id),
        &format!("Duplicate component ID: '{}'", component.id),
        "duplicate",
      ));
    }
  }

  for service in &schema.services {
    if !seen_ids.insert(service.id.as_str()) {
      errors.push(ValidationError::new(
        &format!("services[{}]", service.id),
        &format!("Duplicate service ID: '{}'", service.id),
        "duplicate",
      ));
    }
  }

  for module in &schema.modules {
    if !seen_ids.insert(module.id.as_str()) {
      errors.push(ValidationError::new(
        &format!("modules[{}]", module.id),
        &format!("Duplicate module ID: '{}'", module.id),
        "duplicate",
      ));
    }
  }

  for page in &schema.pages {
    if let Some(ref layout_id) = page.layout {
      if !schema.layouts.iter().any(|l| &l.id == layout_id) {
        errors.push(ValidationError::new(
          &format!("pages.{}.layout", page.id),
          &format!("Referenced layout '{}' not found", layout_id),
          "reference",
        ));
      }
    }
  }

  ValidationResult {
    valid: errors.is_empty(),
    errors,
  }
}

pub fn validate_page(page: &Page, schema: &UiSchema) -> ValidationResult {
  let mut errors: Vec<ValidationError> = Vec::new();

  if page.route.is_empty() {
    errors.push(ValidationError::new(
      &format!("pages.{}.route", page.id),
      "Page route cannot be empty",
      "required",
    ));
  }

  if let Some(ref layout_id) = page.layout {
    if !schema.layouts.iter().any(|l| &l.id == layout_id) {
      errors.push(ValidationError::new(
        &format!("pages.{}.layout", page.id),
        &format!("Referenced layout '{}' not found", layout_id),
        "reference",
      ));
    }
  }

  for element in &page.canvas_elements {
    let element_errors = validate_canvas_element(element, schema);
    errors.extend(element_errors.errors);
  }

  ValidationResult {
    valid: errors.is_empty(),
    errors,
  }
}

pub fn validate_canvas_element(element: &CanvasElement, schema: &UiSchema) -> ValidationResult {
  let mut errors: Vec<ValidationError> = Vec::new();

  if element.id.is_empty() {
    errors.push(ValidationError::new(
      "canvasElement.id",
      "Element ID cannot be empty",
      "required",
    ));
  }

  let component_exists = schema
    .components
    .iter()
    .chain(schema.shared_components.iter())
    .any(|c| c.id == element.component_id);

  if !component_exists {
    errors.push(ValidationError::new(
      &format!("canvasElement.{}", element.id),
      &format!("Referenced component '{}' not found", element.component_id),
      "reference",
    ));
  }

  if element.grid_position.column < 1 {
    errors.push(ValidationError::new(
      &format!("canvasElement.{}.gridPosition.column", element.id),
      "Column must be >= 1",
      "value",
    ));
  }

  if element.grid_position.row < 1 {
    errors.push(ValidationError::new(
      &format!("canvasElement.{}.gridPosition.row", element.id),
      "Row must be >= 1",
      "value",
    ));
  }

  if element.grid_position.col_span < 1 {
    errors.push(ValidationError::new(
      &format!("canvasElement.{}.gridPosition.colSpan", element.id),
      "ColSpan must be >= 1",
      "value",
    ));
  }

  if element.grid_position.row_span < 1 {
    errors.push(ValidationError::new(
      &format!("canvasElement.{}.gridPosition.rowSpan", element.id),
      "RowSpan must be >= 1",
      "value",
    ));
  }

  ValidationResult {
    valid: errors.is_empty(),
    errors,
  }
}

pub fn validate_component(component: &ComponentDef) -> ValidationResult {
  let mut errors: Vec<ValidationError> = Vec::new();

  if component.id.is_empty() {
    errors.push(ValidationError::new(
      &format!("components.{}", component.id),
      "Component ID cannot be empty",
      "required",
    ));
  }

  if component.name.is_empty() {
    errors.push(ValidationError::new(
      &format!("components.{}", component.id),
      "Component name cannot be empty",
      "required",
    ));
  }

  for slot in &component.slots {
    if slot.is_empty() {
      errors.push(ValidationError::new(
        &format!("components.{}.slots", component.id),
        "Slot name cannot be empty",
        "required",
      ));
    }
  }

  for event_name in &component.events {
    if event_name.is_empty() {
      errors.push(ValidationError::new(
        &format!("components.{}.events", component.id),
        "Event name cannot be empty",
        "required",
      ));
    }
  }

  ValidationResult {
    valid: errors.is_empty(),
    errors,
  }
}
