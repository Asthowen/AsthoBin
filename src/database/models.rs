use crate::database::schema::asthobin;
use diesel::{Insertable, Queryable};
use serde::Deserialize;

#[derive(Debug, Clone, Insertable, Deserialize, Queryable)]
#[diesel(table_name = asthobin)]
pub struct AsthoBin {
    pub id: String,
    pub content: String,
    pub language: String,
    pub time: i64,
}
