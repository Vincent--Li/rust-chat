use std::mem;

use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};

use argon2::{Argon2, PasswordVerifier};
use sqlx::PgPool;
use tracing::info;

use crate::AppError;
use crate::User;

impl User {
    pub async fn find_by_email(email: &str, pool: &PgPool) -> Result<Option<Self>, AppError> {
        let user = sqlx::query_as("SELECT id, fullname, email, created_at FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    /// create a new user
    pub async fn create(
        email: &str,
        fullname: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Self, AppError> {
        let password_hash = hash_password(password)?;
        let user = sqlx::query_as("INSERT INTO users (email,fullname,password_hash) VALUES ($1, $2, $3) RETURNING id, fullname, email, created_at")
        .bind(email)
        .bind(fullname)
        .bind(password_hash)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    /// verify email and password
    pub async fn verify(
        email: &str,
        password: &str,
        pool: &PgPool,
    ) -> Result<Option<Self>, AppError> {

        let user: Option<User> = sqlx::query_as(
            "SELECT id, fullname, email, password_hash, created_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        info!("verify user: {:?}", user);

        match user {
            Some(mut user) => {
                let password_hash = mem::take(&mut user.password_hash);
                if verify_password(password, &password_hash.unwrap_or_default())? {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();
    let password_hash = argon2::PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use sqlx_db_tester::TestPg;
    use std::path::Path;

    use super::*;

    #[tokio::test]
    async fn hash_password_should_work() -> Result<()> {
        let password = "password";
        let password_hash = hash_password(password)?;
        assert_eq!(password_hash.len() > 0, true);

        let is_valid = verify_password(password, &password_hash)?;
        assert_eq!(is_valid, true);

        Ok(())
    }

    #[tokio::test]
    async fn create_user_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://vincent:vincent@localhost:5432/chat".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let email = "vincent@gmail.com";
        let fullname = "vincent";
        let password = "password";
        let user = User::create(email, fullname, password, &pool)
            .await
            .unwrap();

        assert_eq!(user.email, "vincent@gmail.com");
        assert_eq!(user.fullname, "vincent");
        assert!(user.id > 0);

        let user = User::find_by_email(email, &pool).await.unwrap();
        assert!(user.is_some());
        let user = user.unwrap();
        assert_eq!(user.email, "vincent@gmail.com");
        assert_eq!(user.fullname, "vincent");

        let user = User::verify(email, password, &pool).await.unwrap();
        assert!(user.is_some());

        Ok(())
    }
}
