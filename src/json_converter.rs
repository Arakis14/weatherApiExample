use serde_json::{Value};

pub fn convert_to_json(s: String) -> serde_json::Value {
  let v: Value = serde_json::from_str(&s).unwrap();
  return v;
}