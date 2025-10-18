use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize,FromRow, Debug, Clone)]
pub struct User{
    pub id: Uuid, 
    pub username: String, 
    pub full_name:String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub created_at: chr
}