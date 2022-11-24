//! src/routes/subscriptions.rs

use actix_web::{
    web::{self, Form},
    HttpResponse,
};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}

pub async fn subscriptions(form: Form<FormData>, connection: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query!(
        r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(connection.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
