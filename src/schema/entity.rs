use nosql_orm::prelude::Entity;
use nosql_orm::EntityMeta;

use super::UiSchema;

impl Entity for UiSchema {
    fn meta() -> EntityMeta {
        EntityMeta::new("ui_schemas")
    }

    fn get_id(&self) -> Option<String> {
        Some(self.app.id.clone())
    }

    fn set_id(&mut self, id: String) {
        self.app.id = id;
    }
}
