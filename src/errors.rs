use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct JsonError {
    err_type: JsonErrorType,
    field: String,
    error: String
}

#[derive(Debug)]
pub struct JsonErrorType {
    code: StatusCode,
    name: &'static str,
}

impl JsonErrorType {
    pub fn get_code(&self) -> StatusCode {
        self.code
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn new_error(self, field: String, msg: String) -> JsonError {
        JsonError::new(self, field, msg)
    }
}

impl JsonError {
    pub fn new(err_type: JsonErrorType, field: String, error: String) -> Self {
        Self { err_type, field, error }
    }
}

impl Display for JsonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            json!({
                "generic": self.err_type.name,
                "error": self.error,
                "field": self.field
            })
        )
    }
}

impl error::ResponseError for JsonError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        self.err_type.code
    }
}

macro_rules! json_error_type {
    ($name:ident, $code:expr) => {
        pub const $name: JsonErrorType = JsonErrorType {
            code: $code,
            name: stringify!($name),
        };
    };
}

#[macro_export]
macro_rules! error {
    ($name:ident, $field:expr, $($arg:tt)*) => {
        crate::errors::$name.new_error(stringify!($field).to_owned(), format!($($arg)*))
    };
}

json_error_type!(NOT_FOUND, StatusCode::NOT_FOUND);
json_error_type!(PARAM_LENGTH, StatusCode::BAD_REQUEST);
json_error_type!(MISSING, StatusCode::BAD_REQUEST);

json_error_type!(DB_CONNECT, StatusCode::INTERNAL_SERVER_ERROR);
json_error_type!(DB_QUERY, StatusCode::INTERNAL_SERVER_ERROR);

json_error_type!(FS_CREATE, StatusCode::INTERNAL_SERVER_ERROR);
json_error_type!(FS_RENAME, StatusCode::INTERNAL_SERVER_ERROR);
json_error_type!(FS_DELETE, StatusCode::INTERNAL_SERVER_ERROR);
json_error_type!(FS_WRITE, StatusCode::INTERNAL_SERVER_ERROR);
json_error_type!(FS_OPEN, StatusCode::INTERNAL_SERVER_ERROR);
json_error_type!(FS_REMOVE, StatusCode::INTERNAL_SERVER_ERROR);

json_error_type!(HASH_MATCH, StatusCode::BAD_REQUEST);

json_error_type!(OVERFLOW, StatusCode::BAD_REQUEST);