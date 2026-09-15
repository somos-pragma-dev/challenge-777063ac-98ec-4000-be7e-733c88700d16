use actix_web::{ResponseError, http::StatusCode};
use diesel::result::Error as DieselError;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LoanError {
    #[error("El préstamo con ID {0} no fue encontrado")]
    NotFound(i64),
    
    #[error("Error de validación: {0}")]
    Validation(String),
    
    #[error("El monto debe ser mayor a cero")]
    InvalidAmount,
    
    #[error("La tasa de interés debe estar entre 0 y 100")]
    InvalidInterestRate,
    
    #[error("La fecha de vencimiento debe ser futura")]
    InvalidDueDate,
    
    #[error("Error de base de datos: {0}")]
    Database(String),
    
    #[error("Error interno del servidor: {0}")]
    Internal(String),
    
    #[error("El préstamo no puede ser modificado en su estado actual")]
    InvalidStateTransition,
    
    #[error("Conflicto de datos: {0}")]
    Conflict(String),
}

impl LoanError {
    pub fn not_found(id: i64) -> Self {
        LoanError::NotFound(id)
    }
    
    pub fn validation(message: impl Into<String>) -> Self {
        LoanError::Validation(message.into())
    }
    
    pub fn invalid_amount() -> Self {
        LoanError::InvalidAmount
    }
    
    pub fn invalid_interest_rate() -> Self {
        LoanError::InvalidInterestRate
    }
    
    pub fn invalid_due_date() -> Self {
        LoanError::InvalidDueDate
    }
    
    pub fn database(error: impl Into<String>) -> Self {
        LoanError::Database(error.into())
    }
    
    pub fn internal(error: impl Into<String>) -> Self {
        LoanError::Internal(error.into())
    }
    
    pub fn invalid_state_transition() -> Self {
        LoanError::InvalidStateTransition
    }
    
    pub fn conflict(message: impl Into<String>) -> Self {
        LoanError::Conflict(message.into())
    }
}

impl fmt::Display for LoanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ResponseError for LoanError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoanError::NotFound(_) => StatusCode::NOT_FOUND,
            LoanError::Validation(_) => StatusCode::BAD_REQUEST,
            LoanError::InvalidAmount => StatusCode::BAD_REQUEST,
            LoanError::InvalidInterestRate => StatusCode::BAD_REQUEST,
            LoanError::InvalidDueDate => StatusCode::BAD_REQUEST,
            LoanError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoanError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoanError::InvalidStateTransition => StatusCode::CONFLICT,
            LoanError::Conflict(_) => StatusCode::CONFLICT,
        }
    }
}

impl From<DieselError> for LoanError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => LoanError::NotFound(0),
            DieselError::DatabaseError(_, info) => {
                LoanError::Database(info.message().to_string())
            }
            _ => LoanError::Database(err.to_string()),
        }
    }
}

impl From<LoanError> for crate::dto::loan_dto::ErrorResponse {
    fn from(err: LoanError) -> Self {
        let message = err.to_string();
        let (code, error) = match &err {
            LoanError::NotFound(_) => ("NOT_FOUND", "Recurso no encontrado"),
            LoanError::Validation(_) => ("VALIDATION_ERROR", "Error de validación"),
            LoanError::InvalidAmount => ("INVALID_AMOUNT", "Monto inválido"),
            LoanError::InvalidInterestRate => ("INVALID_INTEREST_RATE", "Tasa de interés inválida"),
            LoanError::InvalidDueDate => ("INVALID_DUE_DATE", "Fecha de vencimiento inválida"),
            LoanError::Database(_) => ("DATABASE_ERROR", "Error de base de datos"),
            LoanError::Internal(_) => ("INTERNAL_ERROR", "Error interno"),
            LoanError::InvalidStateTransition => ("INVALID_STATE", "Transición de estado inválida"),
            LoanError::Conflict(_) => ("CONFLICT", "Conflicto de datos"),
        };
        crate::dto::loan_dto::ErrorResponse::new(code, error, &message)
    }
}