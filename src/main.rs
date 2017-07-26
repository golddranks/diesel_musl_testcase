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
    PgConnection::establish(&db_url).unwrap();
}
