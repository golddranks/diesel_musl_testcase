#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod models;

fn main() {
    let db_url = std::env::var("DB_URL").unwrap();
    run_migrations(&db_url);
}

pub fn run_migrations(database_url: &str) {
    let conn = PgConnection::establish(database_url).unwrap();
}
