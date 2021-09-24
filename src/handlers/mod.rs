use actix_web::{web, Result, HttpResponse, Responder};
use serde_json::value::Value;

use mongodb::{bson::doc,sync::Client,results::{
    InsertOneResult,
},
};

use crate::utils::build_tree;
use crate::models::Tree;
use crate::db::DB;
use actix_web::error::ResponseError;

use crate::errors::{InvalidDateErrorList, DbError, AppError};

pub async fn index(db: web::Data<DB>,info: web::Json<Value>) -> Result<impl Responder, AppError> {

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
