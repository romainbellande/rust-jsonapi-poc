use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};
pub use std::collections::HashMap;

use super::api::*;

pub trait JApiModel: Serialize
where
    for<'de> Self: Deserialize<'de>,
{
  fn get_type(&self) -> String;

  fn get_id(&self) -> String;

  fn serialize(&self) -> Resource {
    if let Value::Object(mut jsonAttributes) = to_value(self).unwrap() {
      let attributes = jsonAttributes.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

      let resource = Resource {
        id: self.get_id(),
        _type: self.get_type(),
        attributes,
      };

      resource
    } else {
      panic!(format!("{} is not a Value::Object", self.get_type()))
    }
  }
}
