use rand::Rng;
use sqlx::PgPool;
use crate::models::auth::handlers::ErrorGeneratingDiscriminator;
use crate::models::auth::handlers::ErrorGeneratingDiscriminator::{TooManyUsers, DatabaseError};

pub async fn find_unused(pg_pool: &PgPool, username: &String, tries: i64) ->Result<String, ErrorGeneratingDiscriminator>{
    let mut simple_rng = rand::thread_rng();
    let mut tries_buffer = tries;
    loop {
        tries_buffer-=1;
        if tries_buffer < 0
        {
            return Err(TooManyUsers);
        }

        //Create discriminator
        let random_number = simple_rng.gen_range(0..10000);
        let formatted_random_number = format!("{:04}", &random_number);

        //Search if discriminator already exists
        let search_query_result = sqlx::query("SELECT 1 FROM users WHERE username = $1 AND discriminator = $2")
            .bind(username)
            .bind(&formatted_random_number)
            .fetch_all(pg_pool)
            .await;

        match search_query_result {
            Ok(query_users) => {
                if query_users.is_empty(){
                    return Ok(formatted_random_number);
                }
            }
            Err(_) => {
                return Err(DatabaseError);
            }
        }
    };

}