use nosql_orm::prelude::Entity;
use serde::{de::DeserializeOwned, Serialize};

pub trait KernelRepository: Entity + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T: Entity + Serialize + DeserializeOwned + Send + Sync + 'static> KernelRepository for T {}
