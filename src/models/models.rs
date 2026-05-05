
use sqlx::types::Decimal;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct ApiKey{
    pub id: uuid::Uuid,           
    pub user_id: uuid::Uuid,          
    pub exchange : String,       
    pub encrypted_key: Vec<u8>,  
    pub encrypted_secret : Vec<u8>,
    pub created_at: Option<DateTime<Utc>>,      
}

#[derive(Debug, sqlx::FromRow)]
pub struct BotConfig{
    pub id: uuid::Uuid,               
    pub user_id: uuid::Uuid,        
    pub pair: String,            
    pub strategy: String,       
    pub interval_secs: i32,
    pub max_position_pct: Decimal,
    pub active: bool,
    pub updated_at: Option<DateTime<Utc>>,     
}

#[derive(Debug, sqlx::FromRow)]
pub struct BotStat{
    pub user_id: uuid::Uuid,      
    pub status: String,      
    pub crash_count: i32,
    pub updated_at: Option<DateTime<Utc>>,  
}

#[derive(Debug, sqlx::FromRow)]
pub struct Trade{
    pub id: uuid::Uuid,         
    pub user_id: uuid::Uuid,     
    pub pair: String,        
    pub side: String,        
    pub price: Decimal,
    pub quantity: Decimal,
    pub executed_at: Option<DateTime<Utc>>,  
}