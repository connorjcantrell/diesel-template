use crate::schema::users;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub passwd: String,
}