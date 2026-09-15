use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct CreateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: f64,
    
    #[validate(range(min = 0.0, max = 1.0, message = "Interest rate must be between 0 and 1"))]
    pub interest_rate: f64,
    
    #[validate(custom = "validate_future_date")]
    pub due_date: String,
}

fn validate_future_date(date: &str) -> Result<(), validator::ValidationError> {
    let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| validator::ValidationError::new("invalid_date_format"));
    
    match parsed_date {
        Ok(d) => {
            let today = chrono::Utc::now().date_naive();
            if d <= today {
                Err(validator::ValidationError::new("due_date_must_be_future"))
            } else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

#[derive(Validate, Serialize, Deserialize, Debug, Clone)]
pub struct UpdateLoanRequest {
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: Option<f64>,
    
    #[validate(range(min = 0.0, max = 1.0, message = "Interest rate must be between 0 and 1"))]
    pub interest_rate: Option<f64>,
    
    #[validate(custom = "validate_future_date")]
    pub due_date: Option<String>,
    
    #[validate(length(min = 1, max = 20))]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanResponse {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub total_interest: Option<f64>,
    pub total_amount: Option<f64>,
}

impl LoanResponse {
    pub fn from_loan(loan: &crate::models::loan::Loan) -> Self {
        Self {
            id: loan.id,
            amount: loan.amount,
            interest_rate: loan.interest_rate,
            due_date: loan.due_date.format("%Y-%m-%d").to_string(),
            status: loan.status.clone(),
            created_at: loan.created_at.to_rfc3339(),
            updated_at: loan.updated_at.to_rfc3339(),
            total_interest: Some(loan.calculate_total_interest()),
            total_amount: Some(loan.calculate_total_amount()),
        }
    }

    pub fn from_loan_without_calculations(loan: &crate::models::loan::Loan) -> Self {
        Self {
            id: loan.id,
            amount: loan.amount,
            interest_rate: loan.interest_rate,
            due_date: loan.due_date.format("%Y-%m-%d").to_string(),
            status: loan.status.clone(),
            created_at: loan.created_at.to_rfc3339(),
            updated_at: loan.updated_at.to_rfc3339(),
            total_interest: None,
            total_amount: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoanListResponse {
    pub loans: Vec<LoanResponse>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

impl LoanListResponse {
    pub fn new(loans: Vec<LoanResponse>, page: usize, per_page: usize) -> Self {
        Self {
            total: loans.len(),
            loans,
            page,
            per_page,
        }
    }

    pub fn empty(page: usize, per_page: usize) -> Self {
        Self {
            loans: Vec::new(),
            total: 0,
            page,
            per_page,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub code: String,
}

impl ErrorResponse {
    pub fn new(code: &str, error: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            error: error.to_string(),
            message: message.to_string(),
        }
    }

    pub fn validation_error(message: &str) -> Self {
        Self::new("VALIDATION_ERROR", "Validation Failed", message)
    }

    pub fn not_found(id: i64) -> Self {
        Self::new("NOT_FOUND", "Loan Not Found", &format!("Loan with id {} not found", id))
    }

    pub fn internal_error(message: &str) -> Self {
        Self::new("INTERNAL_ERROR", "Internal Server Error", message)
    }

    pub fn bad_request(message: &str) -> Self {
        Self::new("BAD_REQUEST", "Bad Request", message)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
    pub message: Option<String>,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    pub fn with_message(data: T, message: &str) -> Self {
        Self {
            success: true,
            data,
            message: Some(message.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteResponse {
    pub success: bool,
    pub deleted_id: i64,
    pub message: String,
}

impl DeleteResponse {
    pub fn deleted(id: i64) -> Self {
        Self {
            success: true,
            deleted_id: id,
            message: format!("Loan with id {} was successfully deleted", id),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PaginationQuery {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
        }
    }
}

impl PaginationQuery {
    pub fn get_page(&self) -> usize {
        self.page.unwrap_or(1)
    }

    pub fn get_per_page(&self) -> usize {
        let per_page = self.per_page.unwrap_or(10);
        if per_page > 100 {
            100
        } else {
            per_page
        }
    }

    pub fn get_offset(&self) -> usize {
        let page = self.get_page();
        let per_page = self.get_per_page();
        (page.saturating_sub(1)) * per_page
    }
}