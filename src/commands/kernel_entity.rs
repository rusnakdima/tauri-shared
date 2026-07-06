use nosql_orm::prelude::Entity;
use serde::{de::DeserializeOwned, Serialize};

pub trait KernelEntity: Entity + Serialize + DeserializeOwned + Send + Sync + 'static {}

impl<T: Entity + Serialize + DeserializeOwned + Send + Sync + 'static> KernelEntity for T {}