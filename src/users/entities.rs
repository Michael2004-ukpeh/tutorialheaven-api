use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Local, Utc};

#[derive(Deserialize, Serialize,FromRow, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User{
    pub id: Uuid, 
    pub username: String, 
    pub full_name:String,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    pub is_active: bool,
    pub is_verified: bool,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}