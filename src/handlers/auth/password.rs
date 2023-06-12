use std::num::NonZeroU32;
use data_encoding::BASE64;
use ring::{digest, pbkdf2};
use ring::rand::{SecureRandom, SystemRandom};
use crate::models::auth::handlers::ErrorHashPassword;
use crate::models::auth::handlers::ErrorHashPassword::SaltGenerationError;
use crate::models::auth::handlers::ErrorValidatingPassword::InvalidPassword;

pub fn hash(password: &String) -> Result<(String, String), ErrorHashPassword>{

    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let secure_rng = SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    let salt_result = secure_rng.fill(&mut salt);
    match salt_result {
        Ok(_)=>{}
        Err(_)=>{
            return Err(SaltGenerationError);
        }
    }

    let mut hashed_password = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut hashed_password,
    );

    return Ok((BASE64.encode(&hashed_password), BASE64.encode(&salt)))
}

pub fn check(email: &String) -> Result<(), HandlerResponse>
{
    let password_regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap();
    if !password_regex.is_match(&json.password).unwrap()
    {
        return Err(InvalidPassword);
    }
    Ok(())
}
