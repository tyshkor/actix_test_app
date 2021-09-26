use serde_json::value::Value;
use chrono::{DateTime, NaiveDateTime, NaiveDate, NaiveTime, FixedOffset};

use crate::{
    errors::InvalidDateError,
    models::{Tree, TreeValue}
};

pub fn build_tree(serde_json_value: Value) -> (Tree, Vec<InvalidDateError>) {

    let mut tree = Tree::default();
    let mut error_vec = vec![];

    match serde_json_value {
        Value::Null => {
            tree.type_name = "null".to_string();
        },
        Value::Bool(bool) => {
            tree.type_name = "boolean".to_string();
            tree.value = TreeValue::Boolean(bool);
        },
        Value::Number(number) => {
            if number.is_i64() {
                tree.type_name = "integer".to_string();
                tree.value = TreeValue::Integer(number.as_i64().unwrap());

            } else if number.is_f64(){
                tree.type_name = "float".to_string();
                tree.value = TreeValue::Float(number.as_f64().unwrap());

            } else {
                tree.type_name = "number".to_string();
            }
        },
        Value::String(string) => {
            tree.value = TreeValue::String(string);
        },
        Value::Array(vector) => {
            tree.type_name = "array".to_string();
            
            for i in 0..vector.len() {

                let item = vector[i].clone();

                let (mut tree_item, error_list) = build_tree(item);
                tree_item.field = i.to_string();
                let tree_item_field = tree_item.field.clone();
                tree.children.push(Box::new(tree_item));
                    
                for mut error_item in error_list {
                    error_item.form_error_path(tree_item_field.clone());
                    error_vec.push(error_item)
                }
            }
        },
        Value::Object(obj) => {
            tree.type_name = "object".to_string();
            let mut children = vec![];
            for (key, value) in obj {
                if key[..].ends_with("[date]"){
                    match build_date_tree_element(value) {
                        Ok(mut res) => {
                            res.field = key;
                            children.push(Box::new(res));
                        },
                        Err(mut err) => {
                            err.form_error_path(key);
                            error_vec.push(err);
                        }
                    };
                } else {
                    let (mut tree_item, error_list) = build_tree(value);
                    tree_item.field = key;
                    let tree_item_field = tree_item.field.clone();
                    children.push(Box::new(tree_item));

                    for mut error_item in error_list {
                        error_item.form_error_path(tree_item_field.clone());
                        error_vec.push(error_item)
                    }
                }
            }
            tree.children = children;
        },
    }
    (tree, error_vec)
}

fn build_date_tree_element(value: Value) -> Result<Tree, InvalidDateError> {

    let error = InvalidDateError::default();

    match value {
        Value::String(string) => {

            let mut list = vec![];

            for item in string.split("-") {

                let my_item: i32; 
                match item.parse::<i32>(){
                    Ok(value) => {
                        my_item = value;
                    },
                    Err(_) => return Err(error),
                };
                list.push(my_item);
            }

            while list.len() < 3 {
                list.push(1);
            }

            let native_date: NaiveDate;
            
            match NaiveDate::from_ymd_opt(list[0], list[1] as u32, list[2] as u32) {
                Some(nd) => native_date = nd,
                None => return Err(error),
            };

            let native_date_time = NaiveDateTime::new(native_date, NaiveTime::from_hms(0, 0, 0));

            let date_time = DateTime::from_utc(native_date_time, FixedOffset::west(0));

            let mut tree = Tree::default();

            tree.type_name = "date".to_string();
            tree.value = TreeValue::Date(date_time);

            Ok(tree)
        },
        _ => Err(error)
    }
}
