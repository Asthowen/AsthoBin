use crate::database::schema::asthobin;
use serde::Deserialize;

#[derive(Debug, Clone, Insertable, Deserialize, Queryable)]
#[diesel(table_name = asthobin)]
pub struct AsthoBin {
    pub id: String,
    pub content: String,
    pub time: i64,
}
