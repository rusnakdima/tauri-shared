use crate::schema::i18n::I18nConfig;

pub fn translate(key: &str, locale: &str, config: &I18nConfig) -> String {
  let Some(locale_map) = config.locales.get(locale) else {
    return key.to_string();
  };

  if let Some(value) = locale_map.nav.get(key) {
    return value.clone();
  }
  if let Some(value) = locale_map.actions.get(key) {
    return value.clone();
  }
  if let Some(value) = locale_map.messages.get(key) {
    return value.clone();
  }

  key.to_string()
}

#[cfg(feature = "tauri")]
#[tauri::command]
pub fn tauri_translate(
  key: String,
  locale: String,
  config: tauri::State<'_, I18nConfig>,
) -> String {
  translate(&key, &locale, &config)
}
