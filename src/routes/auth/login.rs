use std::num::NonZeroU32;
use actix_web::{post, Responder, HttpResponse, web};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use fancy_regex::Regex;
use ring::pbkdf2;
use ring::rand::{SecureRandom, SystemRandom};
use data_encoding::BASE64;
use sqlx::Row;
use crate::models::application::AppState;

#[derive(Deserialize)]
pub struct LoginReq{
    email: String,
    password: String,
}

#[derive(sqlx::FromRow)]
pub struct CredentialsRow{
    id: i64,
    salt: String,
    password: String,
}


#[post("/auth/login")]
pub async fn post_auth_login(data: web::Data<AppState>, json: web::Json<LoginReq>) -> impl Responder {

    let credentials_request = sqlx::query_as::<_,CredentialsRow>("select id, salt, password from users where email = $1")
        .bind(&json.email)
        .fetch_all(&data.pg_pool)
        .await;
    let credentials = match credentials_request {
        Ok(v) =>{
            if v.is_empty(){
                return HttpResponse::BadRequest().json(SimpleLoginRes {error: true, message: "Failed to find user".to_string()});
            }

        }
        Err(_)=>{
            return HttpResponse::InternalServerError().json(SimpleLoginRes {error: true, message: "Failed to find user".to_string()});
        }
    };

    let n_iter = NonZeroU32::new(100_000).unwrap();

    let found_user_id = credentials.id;
    let found_user_salt_result = BASE64.decode(credentials.salt.as_bytes());
    let found_user_salt = match found_user_salt_result {
        Ok(v) =>{v}
        Err(_)=>{
            return HttpResponse::InternalServerError().json(SimpleLoginRes {error: true, message: "Failed to decode salt".to_string()});
        }
    };
    let found_user_password_result = BASE64.decode(credentials.password.as_bytes());
    let found_user_password = match found_user_password_result {
        Ok(v) =>{v}
        Err(_)=>{
            return HttpResponse::InternalServerError().json(SimpleLoginRes {error: true, message: "Failed to decode password".to_string()});
        }
    };

    let password_verification = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &found_user_salt,
        &json.password.as_bytes(),
        &found_user_password,
    );

    if !password_verification.is_ok()
    {
        return HttpResponse::BadRequest().json(SimpleLoginRes {error: true, message: "Incorrect Credentials".to_string()});
    }

    let logged_request = sqlx::query("select 1 from tokens where user_id = $1")
        .bind(&found_user_id)
        .fetch_all(&data.pg_pool)
        .await;
    match logged_request {
        Ok(_) =>{}
        Err(_)=>{
            return HttpResponse::InternalServerError().json(SimpleLoginRes {error: true, message: "Already logged in".to_string()});
        }
    }

    //Create user token
    const TOKEN_STRING_SIZE: usize = 64;
    let secure_rng = SystemRandom::new();
    let token_id = Utc::now().timestamp_millis();
    let created_date = Utc::now();

    let mut token_string = [0u8; TOKEN_STRING_SIZE];
    let token_string_response = secure_rng.fill(&mut token_string);
    match token_string_response {
        Ok(_)=>{}
        Err(_)=>{
            return HttpResponse::InternalServerError().json(SimpleLoginRes {error: true, message: "Failed to create token".to_string()});
        }
    }

    let token = format!("{}{}", BASE64.encode(&found_user_id.to_be_bytes()), BASE64.encode(&token_string));

    let insert_token_response = sqlx::query("INSERT INTO tokens(id, user_id, token, created_date) values($1, $2, $3, $4)")
        .bind(&token_id)
        .bind(&found_user_id)
        .bind(&token)
        .bind(&created_date)
        .execute(&data.pg_pool)
        .await;
    return match insert_token_response {
        Ok(_) => {
            HttpResponse::Ok().json(CompleteLoginRes{error:false, message: "Logged in Successfully".to_string(), token })
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(SimpleLoginRes { error: true, message: "Failed to insert token to database".to_string() })
        }
    }
}