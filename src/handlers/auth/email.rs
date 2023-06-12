use fancy_regex::Regex;
use sqlx::PgPool;
use crate::models::auth::handlers::{ErrorUniqueEmail, ErrorValidatingEmail};
use crate::models::auth::handlers::ErrorUniqueEmail::{DatabaseError, EmailNotUnique};
use crate::models::auth::handlers::ErrorValidatingEmail::InvalidEmail;

pub fn check(email: &String) -> Result<(), ErrorValidatingEmail>
{
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_regex.is_match(email).unwrap()
    {
        return Err(InvalidEmail);
    }
    return Ok(());
}

pub async fn database_unique(pg_pool: &PgPool, email: &String) -> Result<(), ErrorUniqueEmail>
{
    let matching_emails_result = sqlx::query("SELECT 1 FROM users WHERE email = $1")
        .bind(email)
        .fetch_all(pg_pool)
        .await;
    return match matching_emails_result {
        Ok(query_users) => {
            if !query_users.is_empty() {
                return Err(EmailNotUnique);
            }
            Ok(())
        }
        Err(_) => {
            Err(DatabaseError)
        }
    }
}