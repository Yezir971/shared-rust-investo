pub mod crypto;
pub mod repositories;
pub mod schema;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub async fn run_migrations(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    sqlx::migrate!()
        .run(pool)
        .await?;
        
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
