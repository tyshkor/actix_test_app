use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum TreeValue {
    String(String),
    Date(DateTime<FixedOffset>),
    Boolean(bool),
    Integer(i64),
    Float(f64),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tree {
    pub field: String,   
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_name: String,
    pub value: TreeValue,
    pub children: Vec<Box<Tree>>,
}

impl Tree {

    pub fn default() -> Tree {
        Tree {
            field: "".to_string(),
            type_name: "".to_string(),
            value: TreeValue::String("".to_string()),
            children: vec![],
        }
    }
}