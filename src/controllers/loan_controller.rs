use actix_web::{web, HttpResponse, Result, ResponseError};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::dto::loan_dto::{LoanRequest, LoanResponse, ApiResponse, PaginatedResponse};
use crate::error::loan_error::LoanError;
use crate::services::loan_service::LoanService;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be positive"))]
    pub amount: f64,
    
    #[validate(range(min = 0.0, max = 100.0, message = "Interest rate must be between 0 and 100"))]
    pub interest_rate: f64,
    
    #[validate(regex(path = "*", message = "Due date must be in YYYY-MM-DD format"))]
    pub due_date: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be positive"))]
    pub amount: Option<f64>,
    
    #[validate(range(min = 0.0, max = 100.0, message = "Interest rate must be between 0 and 100"))]
    pub interest_rate: Option<f64>,
    
    #[validate(regex(path = "*", message = "Due date must be in YYYY-MM-DD format"))]
    pub due_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanPathParams {
    pub id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: usize,
    
    #[serde(default = "default_per_page")]
    pub per_page: usize,
}

fn default_page() -> usize { 1 }
fn default_per_page() -> usize { 10 }

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/loans")
            .route("", web::get().to(list_loans))
            .route("", web::post().to(create_loan))
            .route("/{id}", web::get().to(get_loan))
            .route("/{id}", web::put().to(update_loan))
            .route("/{id}", web::delete().to(delete_loan))
            .route("/{id}/activate", web::post().to(activate_loan))
            .route("/{id}/deactivate", web::post().to(deactivate_loan))
            .route("/{id}/mark-paid", web::post().to(mark_as_paid))
            .route("/{id}/mark-defaulted", web::post().to(mark_as_defaulted))
    );
}

async fn list_loans(
    service: web::Data<LoanService>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse, LoanError> {
    let page = query.page;
    let per_page = query.per_page;
    
    if per_page > 100 {
        return Err(LoanError::BadRequest("Per page cannot exceed 100".to_string()));
    }
    
    let result = service.list_loans(page, per_page).await?;
    Ok(HttpResponse::Ok().json(result))
}

async fn get_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.get_loan(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

async fn create_loan(
    service: web::Data<LoanService>,
    body: web::Json<CreateLoanRequest>,
) -> Result<HttpResponse, LoanError> {
    body.validate().map_err(|e| {
        LoanError::ValidationError(e.to_string())
    })?;
    
    let loan = service.create_loan(
        body.amount,
        body.interest_rate,
        &body.due_date,
    ).await?;
    
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Created().json(ApiResponse::new(response)))
}

async fn update_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
    body: web::Json<UpdateLoanRequest>,
) -> Result<HttpResponse, LoanError> {
    body.validate().map_err(|e| {
        LoanError::ValidationError(e.to_string())
    })?;
    
    let loan = service.update_loan(
        path.id,
        body.amount,
        body.interest_rate,
        body.due_date.as_deref(),
    ).await?;
    
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::new(response)))
}

async fn delete_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    service.delete_loan(path.id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::deleted(path.id)))
}

async fn activate_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.activate_loan(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan activated successfully")))
}

async fn deactivate_loan(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.deactivate_loan(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan deactivated successfully")))
}

async fn mark_as_paid(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.mark_as_paid(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan marked as paid")))
}

async fn mark_as_defaulted(
    service: web::Data<LoanService>,
    path: web::Path<LoanPathParams>,
) -> Result<HttpResponse, LoanError> {
    let loan = service.mark_as_defaulted(path.id).await?;
    let response = LoanResponse::from_loan(&loan);
    Ok(HttpResponse::Ok().json(ApiResponse::with_message(response, "Loan marked as defaulted")))
}