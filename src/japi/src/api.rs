pub use std::collections::HashMap;
use serde_json::{Value};

#[derive(Serialize)]
pub struct Resource {
  pub id: String,
  #[serde(rename = "type")]
  pub _type: String,
  pub attributes: ResourceAttributes,
}

#[derive(Deserialize, Debug)]
pub struct DocumentDto<T> {
  pub data: ResourceDto<T>,
}

impl<T> DocumentDto<T> {
  pub fn deserialize(&self) -> &T {
    &self.data.attributes
  }
}

#[derive(Deserialize, Debug)]
pub struct ResourceDto<T> {
  #[serde(rename = "type")]
  pub _type: String,
  pub attributes: T,
}

pub struct Relationship {
  pub data: IdentifierData,
}

#[derive(Serialize)]
pub struct ResourceIdentifier {
  pub id: String,
  #[serde(rename = "type")]
  pub _type: String,
}

pub enum IdentifierData {
  None,
  Single(ResourceIdentifier),
  Multiple(Vec<ResourceIdentifier>),
}

pub type ResourceAttributes = HashMap<String, Value>;
