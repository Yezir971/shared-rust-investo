use sqlx::PgPool;
use uuid::Uuid;
use crate::crypto; 
use anyhow::Result;
use crate::schema::key_api::DecryptedKey;

// fonction pour envoyer en base de données les clés api des utilisateurs 
pub async fn save_api_key(
    pool: &PgPool,
    user_id: Uuid,
    exchange: &str,
    raw_key: &str,
    raw_secret: &str,
    master_key: &[u8]
) -> Result<()> {
    // Chiffrement des données sensibles
    let encrypted_key = crypto::encrypt(raw_key, master_key)?;
    let encrypted_secret = crypto::encrypt(raw_secret, master_key)?;

    sqlx::query!(
        r#"
        INSERT INTO api_keys (user_id, exchange, encrypted_key, encrypted_secret)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        exchange,
        encrypted_key,
        encrypted_secret
    )
    .execute(pool)
    .await?;

    Ok(())
}

// fonction pour que le bot puisse récupérer les données 
pub async fn get_decrypted_keys(
    pool: &PgPool,
    user_id: Uuid,
    master_key: &[u8]
) -> Result<Vec<DecryptedKey>> {
    let rows = sqlx::query!(
        "SELECT exchange, encrypted_key, encrypted_secret FROM api_keys WHERE user_id = $1",
        user_id
    )
    .fetch_all(pool)
    .await?;

    let mut decrypted_list = Vec::new();

    for row in rows {
        let key = crypto::decrypt(&row.encrypted_key, master_key)?;
        let secret = crypto::decrypt(&row.encrypted_secret, master_key)?;
        
        decrypted_list.push(DecryptedKey {
            exchange: row.exchange,
            key,
            secret,
        });
    }

    Ok(decrypted_list)
}