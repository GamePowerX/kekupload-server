use crate::diesel::RunQueryDsl;
use diesel::{PgConnection, QueryDsl, QueryResult};
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

use crate::schema::files;

use crate::diesel::ExpressionMethods;

#[derive(Queryable, Insertable)]
#[diesel(table_name = files)]
pub struct File {
    pub id: String,
    pub ext: String,
    pub hash: String,
}

impl File {
    pub fn create(&self, connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> QueryResult<usize> {
        diesel::insert_into(files::table)
            .values(self)
            .execute(connection)
    }

    pub fn find(id: String, connection: &mut PooledConnection<ConnectionManager<PgConnection>>) -> QueryResult<Vec<File>> {
        files::table
            .filter(files::dsl::id.eq(id))
            .select((files::dsl::id, files::dsl::ext, files::dsl::hash))
            .load::<File>(connection)
    }
}
