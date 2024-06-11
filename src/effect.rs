use serde_json::Value;

#[derive(Debug)]
pub struct Effect {
    pub(crate) name: String,
    pub(crate) matrix: Vec<Vec<f64>>,
    pub(crate) options: Value, //TODO
}