
use crate::schema::bot_api::SoldeUser;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;



pub async fn give_solde(pool: &PgPool, user_id: Uuid) -> Result<SoldeUser> {

    let row = sqlx::query!(
        r#"SELECT virtual_balance as "virtual_balance!" FROM users WHERE id = $1"#,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(SoldeUser {
        virtual_balance: row.virtual_balance,
        id_user: user_id,
    })
}