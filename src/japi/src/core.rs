pub use super::model::JApiModel;

#[macro_export]
macro_rules! japi_model {
  ($model:ty; $type:expr) => {
    impl JApiModel for $model {
      fn get_id(&self) -> String { self.id.to_string() }
      fn get_type(&self) -> String { $type.to_string() }
    }
  }
}
