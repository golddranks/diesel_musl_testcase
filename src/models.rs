use super::schema::*;
use chrono::{DateTime};
use chrono::offset::Utc;

#[derive(Queryable, Identifiable, Insertable, AsChangeset, PartialEq, Eq, Hash, Debug, Serialize,
         Deserialize)]
#[table_name = "members"]
pub struct Member {
    pub id: String,
    pub employee_num: Option<i32>,
    pub name: String,
    pub leader: bool,
    pub mail: String,
    pub phone: String,
    pub business_connection: String,
    pub memo: String,
    pub joined: DateTime<Utc>,
}
