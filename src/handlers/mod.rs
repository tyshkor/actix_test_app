use actix_web::{web::{Data, Json}, Result, HttpResponse, Responder};
use serde_json::value::Value;

use crate::{
    utils::build_tree,
    db::DB,
    errors::{InvalidDateErrorList, DbError, AppError}
};

pub async fn post_json_to_tree(db: Data<DB>,info: Json<Value>) -> Result<impl Responder, AppError> {

    let serde_json_value = info.into_inner();

    let (tree, error_list) = build_tree(serde_json_value);

    if error_list.len() > 0 {
        
        return Err(AppError::InvalidData(InvalidDateErrorList(error_list)))
    }

    if let Ok(_) = db.save(&tree) {
        Ok(HttpResponse::Created().json(tree.children))
    } else {
        Err(AppError::DbError(DbError::default()))
    }

}
