use chrono::Utc;
use data_encoding::BASE64;
use ring::rand::{SecureRandom, SystemRandom};
use sqlx::PgPool;
use crate::models::auth::handlers::ErrorGeneratingToken;
use crate::models::auth::handlers::ErrorGeneratingToken::{DatabaseError, TokenGenerationError};

pub fn generate_token(user_id: &String) -> Result<String, ErrorGeneratingToken>
{
    //Create user token
    const TOKEN_STRING_SIZE: usize = 64;
    let secure_rng = SystemRandom::new();

    let mut token_string = [0u8; TOKEN_STRING_SIZE];
    let token_string_response = secure_rng.fill(&mut token_string);
    match token_string_response {
        Ok(_)=>{}
        Err(_)=>{
            return Err(TokenGenerationError);
        }
    }

    let token = format!("{}{}", BASE64.encode(user_id.to_be_bytes()), BASE64.encode(&token_string));
    Ok(token)
}

pub fn generate_id() -> i64
{
    let token_id = Utc::now().timestamp_millis();

    return token_id;
}

pub async fn insert(pg_pool: PgPool, token: &String, user_id: &String) -> Result<(), ErrorGeneratingToken>
{
    let created_date = Utc::now();
    let token_id = generate_id();

    let insert_token_response = sqlx::query("INSERT INTO tokens(id, user_id, token, created_date) values($1, $2, $3, $4)")
        .bind(&token_id)
        .bind(&user_id)
        .bind(&token)
        .bind(&created_date)
        .execute(pg_pool)
        .await;

    return match insert_token_response {
        Ok(_) => {
            Ok(())
        }
        Err(_) => {
            Err(DatabaseError)
        }
    }
}