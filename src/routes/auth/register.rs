use actix_web::{post, Responder, HttpResponse, web};
use crate::AppState;

use rand::Rng;
use chrono::Utc;
use data_encoding::BASE64;
use ring::rand::{SecureRandom, SystemRandom};
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;
use serde::{Deserialize, Serialize};
use fancy_regex::Regex;

use crate::prelude::auth::*;

#[derive(Deserialize)]
pub struct RegisterReq {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct SimpleRegisterRes {
    error: bool,
    message: String,
}

#[derive(Serialize)]
pub struct CompleteRegisterRes {
    error: bool,
    message: String,
    token: String,
}

#[post("/auth/register")]
pub async fn post_auth_register(data: web::Data<AppState>, json: web::Json<RegisterReq>) -> impl Responder {

    let insert_user_query_result = sqlx::query("INSERT INTO USERS(id, username, discriminator, email, salt, password, created_date) values($1, $2, $3, $4, $5, $6, $7)")
        .bind(&id)
        .bind(&json.username)
        .bind(&discriminator)
        .bind(&json.email)
        .bind(salt)
        .bind(hashed_password)
        .bind(&created_date)
        .execute(&data.pg_pool)
        .await;
    match insert_user_query_result {
        Ok(_)=>{}
        Err(_)=>{
            return HttpResponse::InternalServerError().json(SimpleRegisterRes {error: true, message: "Failed to insert user to database".to_string()});
        }
    }
}