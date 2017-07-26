#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod models;

use models::*;
use schema::*;

fn main() {
    let db_url = std::env::var("DB_URL").unwrap();
    run_migrations(&db_url);
}

embed_migrations!();

pub fn run_migrations(database_url: &str) {
    let conn = PgConnection::establish(database_url).unwrap();
    embedded_migrations::run(&conn).unwrap();
}
