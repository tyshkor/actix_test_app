use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, NaiveDate, NaiveTime, FixedOffset};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum TreeValue {
    string(String),
    date(DateTime<chrono::FixedOffset>),
    boolean(bool),
    integer(i64),
    float(f64),
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
            value: TreeValue::string("".to_string()),
            children: vec![],
        }
    }
}