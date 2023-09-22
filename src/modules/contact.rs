use serde::{Deserialize, Serialize};
use sqlx::FromRow;
#[derive(FromRow, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: Option<i32>,
    pub name: String,
    pub last_name: Option<String>,
    pub birthday: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub notes: Option<String>,
}
