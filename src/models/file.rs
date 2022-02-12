use diesel::{PgConnection, QueryResult, QueryDsl};
use crate::diesel::RunQueryDsl;

use crate::schema::files;

use crate::diesel::ExpressionMethods;

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

    pub fn find(id: String, connection: &PgConnection) -> Vec<File> {
        files::table
            .filter(files::dsl::id.eq(id))
            .select((files::dsl::id, files::dsl::ext, files::dsl::hash))
            .load::<File>(connection)
            .expect("Error while executing query!")
    }
}