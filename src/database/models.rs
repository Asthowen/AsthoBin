use diesel::{Insertable, Queryable};
use serde::Deserialize;

use super::schema::asthobin;

#[derive(Clone, Debug, Deserialize, Insertable, Queryable)]
#[diesel(table_name = asthobin)]
pub struct AsthoBin {
    pub id: String,
    pub content: String,
    pub language: String,
    pub time: i64,
}
