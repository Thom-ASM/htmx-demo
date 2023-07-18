use sqlx::{postgres::PgArguments, query::Query, Postgres};

pub mod mutations;
pub mod queries;
type QueryT<'a> = Query<'a, Postgres, PgArguments>;
