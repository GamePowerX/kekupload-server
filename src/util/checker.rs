use diesel::{QueryResult, r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

use crate::{database::PgPool, errors::JsonError};

pub fn in_bounds<'a, 'b>(param: &'b str, len: usize, min: usize, max: usize) -> Result<(), JsonError> {
    if len < min || len > max {
        return Err(crate::error!(PARAM_LENGTH, param, "{} must be in bounds {}-{}!", param, min, max));
    }
    Ok(())
}

pub fn map_qres<'a, T>(qres: QueryResult<T>, text: &'a str) -> Result<T, JsonError> {
    qres.map_err(|e| crate::error!(DB_QUERY, QUERY, "{}: {}", text, e))
}

pub fn map_opt<'a, T>(opt: Option<T>, msg: &'a str) -> Result<T, JsonError> {
    match opt {
        Some(val) => Ok(val),
        None => Err(crate::error!(MISSING, OPTIONAL, "{}", msg)),
    }
}

pub fn get_con(
    pool: &PgPool,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, JsonError> {
    pool.get().map_err(|e| crate::error!(DB_CONNECT, CONNECTION, "Error while getting connection from pool: {}", e))
}