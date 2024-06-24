use serde::Serialize;

pub fn filter_db_record<T: Serialize>(model: &T) -> serde_json::Value {
    serde_json::to_value(model).unwrap()
}
