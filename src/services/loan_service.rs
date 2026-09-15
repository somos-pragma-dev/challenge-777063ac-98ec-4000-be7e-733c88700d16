use crate::models::loan::{Loan, LoanStatus};
use crate::dto::loan_dto::{LoanResponse, PaginatedLoansResponse, ApiResponse, ErrorResponse};
use crate::repositories::loan_repository::LoanRepository;
use crate::error::loan_error::LoanError;
use chrono::NaiveDate;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct LoanService<R: LoanRepository> {
    repository: Arc<R>,
}

impl<R: LoanRepository> LoanService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }
    
    pub fn create_loan(
        &self,
        amount: f64,
        interest_rate: f64,
        due_date: NaiveDate,
    ) -> Result<ApiResponse<LoanResponse>, LoanError> {
        self.validate_loan_data(amount, interest_rate, due_date)?;
        
        let loan = Loan::new(0, amount, interest_rate, due_date);
        loan.validate().map_err(LoanError::validation)?;
        
        let created = self.repository.create(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&created);
        Ok(ApiResponse::new(response))
    }
    
    pub fn get_loan(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        let response = LoanResponse::from_loan(&loan);
        Ok(ApiResponse::new(response))
    }
    
    pub fn list_loans(&self, page: usize, per_page: usize) -> Result<PaginatedLoansResponse, LoanError> {
        if page == 0 {
            return Err(LoanError::validation("El número de página debe ser mayor a 0"));
        }
        if per_page == 0 {
            return Err(LoanError::validation("Los elementos por página deben ser mayor a 0"));
        }
        
        let loans = self.repository.find_all(page, per_page)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let total = self.repository.count()
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let responses: Vec<LoanResponse> = loans.iter()
            .map(LoanResponse::from_loan)
            .collect();
        
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as usize;
        
        if responses.is_empty() {
            return Ok(PaginatedLoansResponse::empty(page, per_page));
        }
        
        Ok(PaginatedLoansResponse::new(responses, page, per_page))
    }
    
    pub fn update_loan(
        &self,
        id: i64,
        amount: Option<f64>,
        interest_rate: Option<f64>,
        due_date: Option<NaiveDate>,
    ) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if !loan.is_active() {
            return Err(LoanError::invalid_state_transition());
        }
        
        if let Some(new_amount) = amount {
            if new_amount <= 0.0 {
                return Err(LoanError::invalid_amount());
            }
            loan.update_amount(new_amount)
                .map_err(LoanError::validation)?;
        }
        
        if let Some(new_rate) = interest_rate {
            if new_rate < 0.0 || new_rate > 100.0 {
                return Err(LoanError::invalid_interest_rate());
            }
            loan.update_interest_rate(new_rate)
                .map_err(LoanError::validation)?;
        }
        
        if let Some(new_due_date) = due_date {
            let today = chrono::Local::now().date_naive();
            if new_due_date <= today {
                return Err(LoanError::invalid_due_date());
            }
            loan.update_due_date(new_due_date)
                .map_err(LoanError::validation)?;
        }
        
        loan.validate().map_err(LoanError::validation)?;
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn delete_loan(&self, id: i64) -> Result<ApiResponse<()>, LoanError> {
        let loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if loan.is_active() {
            return Err(LoanError::conflict("No se puede eliminar un préstamo activo"));
        }
        
        self.repository.delete(id)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        Ok(ApiResponse::new(()))
    }
    
    pub fn activate_loan(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        loan.activate();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn deactivate_loan(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        loan.deactivate();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn mark_as_paid(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if !loan.is_active() {
            return Err(LoanError::invalid_state_transition());
        }
        
        loan.mark_as_paid();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    pub fn mark_as_defaulted(&self, id: i64) -> Result<ApiResponse<LoanResponse>, LoanError> {
        let mut loan = self.repository.find_by_id(id)
            .map_err(|e| match e {
                LoanError::NotFound(_) => LoanError::not_found(id),
                _ => LoanError::internal(e.to_string()),
            })?;
        
        if !loan.is_active() {
            return Err(LoanError::invalid_state_transition());
        }
        
        loan.mark_as_defaulted();
        
        let updated = self.repository.update(loan)
            .map_err(|e| LoanError::internal(e.to_string()))?;
        
        let response = LoanResponse::from_loan(&updated);
        Ok(ApiResponse::new(response))
    }
    
    fn validate_loan_data(
        &self,
        amount: f64,
        interest_rate: f64,
        due_date: NaiveDate,
    ) -> Result<(), LoanError> {
        if amount <= 0.0 {
            return Err(LoanError::invalid_amount());
        }
        
        if interest_rate < 0.0 || interest_rate > 100.0 {
            return Err(LoanError::invalid_interest_rate());
        }
        
        let today = chrono::Local::now().date_naive();
        if due_date <= today {
            return Err(LoanError::invalid_due_date());
        }
        
        Ok(())
    }
}