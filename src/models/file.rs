use diesel::{PgConnection, QueryResult};
use crate::diesel::RunQueryDsl;

use crate::schema::files;

#[derive(Queryable, Insertable)]
#[table_name="files"]
pub struct File {
    pub id: String,
    pub ext: String,
    pub hash: String
}

impl File {
    pub fn create(&self, connection: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(files::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: String) {
        
    }
}