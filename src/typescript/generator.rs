use ts_rs::TS;

pub fn generate_typescript_bindings() {
  println!("cargo:rerun-if-changed=src/schema");
}

#[macro_export]
macro_rules! derive_ts {
    ($($t:ty),*) => {
        $(
            const _: () = assert_ts!($t);
        )*
    };
}

pub trait ToTypeScript {
  fn to_typescript() -> String;
}

impl<T: TS + 'static> ToTypeScript for T {
  fn to_typescript() -> String {
    T::inline()
  }
}

pub fn ts_inline<T: TS + 'static>() -> String {
  T::inline()
}

pub fn schema_ts_bindings() -> String {
  let mut bindings = String::new();

  bindings.push_str(&crate::schema::UiSchema::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::Page::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::PageMeta::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::PageSection::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::CanvasElement::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::GridPosition::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::DataBinding::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::AppConfig::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::AppSettings::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::ComponentDef::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::ComponentProp::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::GridDefaults::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::TailwindBreakpoints::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::TailwindGridArea::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::TailwindGridElement::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::TailwindResponsiveClasses::inline());
  bindings.push('\n');

  // Runtime schema types (in src/schema/runtime.rs)
  bindings.push_str(&crate::schema::runtime::RenderedPage::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::runtime::RenderedSection::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::runtime::RenderedElement::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::runtime::RenderedCanvasElement::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::runtime::ValidationResult::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::runtime::ValidationError::inline());
  bindings.push('\n');
  bindings.push_str(&crate::schema::runtime::EventSignature::inline());
  bindings.push('\n');

  // Runtime router types (in src/runtime/router.rs)
  bindings.push_str(&crate::runtime::RouteConfig::inline());
  bindings.push('\n');
  bindings.push_str(&crate::runtime::RouteMatch::inline());
  bindings.push('\n');

  bindings
}
