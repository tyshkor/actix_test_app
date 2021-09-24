use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};
use actix_web::{HttpResponse, http::StatusCode, error::ResponseError};

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum AppError {
    
    InvalidData(InvalidDateErrorList),

    DbError(DbError),
}

impl Display for AppError {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &*self {
            AppError::InvalidData(err) =>  write!(f, "{:?}", err),
            AppError::DbError(err) => write!(f, "{:?}", err),
        }
    }
}

impl ResponseError for AppError {

    fn status_code(&self) -> StatusCode {
        match &*self {
            AppError::InvalidData(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
        
    }
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}


#[derive(Deserialize, Serialize, Debug)]
pub struct InvalidDateError {
    pub msg: String,
    pub path: String,
}

impl InvalidDateError {

    pub fn default() -> InvalidDateError {

        InvalidDateError {
            msg: "Invalid date".to_string(),
            path: "".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct InvalidDateErrorList(pub Vec<InvalidDateError>);

impl Display for InvalidDateErrorList {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DbError {
    pub msg: String,
}

impl DbError {

    pub fn default() -> DbError {

        DbError {
            msg: "database error".to_string(),
        }
    }
}

impl Display for DbError {

    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?}", self)
    }
}
