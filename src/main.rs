use actix_web::{web, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use std::env;

mod schema;
mod models;
mod dto;
mod services;
mod repositories;
mod controllers;
mod error;

use controllers::loan_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("SERVER_PORT must be a valid number");
    
    println!("Starting server at http://{}:{}", host, port);
    
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                services::loan_service::LoanService::new(
                    repositories::loan_repository::PgLoanRepository::new()
                )
            ))
            .route("/health", web::get().to(health_check))
            .configure(loan_controller::configure)
    })
    .bind((host, port))?
    .run()
    .await
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "loan-api",
        "version": "1.0.0"
    })))
}