use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Utc};
use std::fmt;

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::loans)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Loan {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: NaiveDate,
    pub status: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl Loan {
    pub fn new(id: i64, amount: f64, interest_rate: f64, due_date: NaiveDate) -> Self {
        let now = Utc::now();
        Self {
            id,
            amount,
            interest_rate,
            due_date,
            status: LoanStatus::Pending.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_status(mut self, status: LoanStatus) -> Self {
        self.status = status.to_string();
        self.updated_at = Utc::now();
        self
    }

    pub fn is_active(&self) -> bool {
        self.status == LoanStatus::Active.to_string()
    }

    pub fn is_overdue(&self) -> bool {
        if self.status != LoanStatus::Active.to_string() {
            return false;
        }
        let today = Utc::now().date_naive();
        self.due_date < today
    }

    pub fn calculate_total_interest(&self) -> f64 {
        let days = self.days_until_due();
        let daily_rate = self.interest_rate / 365.0;
        self.amount * daily_rate * days as f64
    }

    pub fn calculate_total_amount(&self) -> f64 {
        self.amount + self.calculate_total_interest()
    }

    fn days_until_due(&self) -> i64 {
        let today = Utc::now().date_naive();
        if self.due_date > today {
            (self.due_date - today).num_days()
        } else {
            0
        }
    }

    pub fn update_amount(&mut self, new_amount: f64) -> Result<(), String> {
        if new_amount <= 0.0 {
            return Err("Loan amount must be positive".to_string());
        }
        self.amount = new_amount;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_interest_rate(&mut self, new_rate: f64) -> Result<(), String> {
        if new_rate < 0.0 || new_rate > 1.0 {
            return Err("Interest rate must be between 0.0 and 1.0".to_string());
        }
        self.interest_rate = new_rate;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_due_date(&mut self, new_due_date: NaiveDate) -> Result<(), String> {
        let today = Utc::now().date_naive();
        if new_due_date <= today {
            return Err("Due date must be in the future".to_string());
        }
        self.due_date = new_due_date;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn activate(&mut self) {
        self.status = LoanStatus::Active.to_string();
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.status = LoanStatus::Inactive.to_string();
        self.updated_at = Utc::now();
    }

    pub fn mark_as_paid(&mut self) {
        self.status = LoanStatus::Paid.to_string();
        self.updated_at = Utc::now();
    }

    pub fn mark_as_defaulted(&mut self) {
        self.status = LoanStatus::Defaulted.to_string();
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoanStatus {
    Pending,
    Active,
    Inactive,
    Paid,
    Defaulted,
}

impl LoanStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            LoanStatus::Pending => "pending",
            LoanStatus::Active => "active",
            LoanStatus::Inactive => "inactive",
            LoanStatus::Paid => "paid",
            LoanStatus::Defaulted => "defaulted",
        }
    }
}

impl fmt::Display for LoanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for LoanStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(LoanStatus::Pending),
            "active" => Ok(LoanStatus::Active),
            "inactive" => Ok(LoanStatus::Inactive),
            "paid" => Ok(LoanStatus::Paid),
            "defaulted" => Ok(LoanStatus::Defaulted),
            _ => Err(format!("Unknown loan status: {}", s)),
        }
    }
}

impl Default for LoanStatus {
    fn default() -> Self {
        LoanStatus::Pending
    }
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::loans)]
pub struct NewLoan {
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: NaiveDate,
}

impl NewLoan {
    pub fn validate(&self) -> Result<(), String> {
        if self.amount <= 0.0 {
            return Err("Amount must be greater than zero".to_string());
        }
        if self.interest_rate < 0.0 || self.interest_rate > 1.0 {
            return Err("Interest rate must be between 0 and 1 (0% to 100%)".to_string());
        }
        let today = Utc::now().date_naive();
        if self.due_date <= today {
            return Err("Due date must be in the future".to_string());
        }
        Ok(())
    }
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::loans)]
pub struct UpdateLoan {
    pub amount: Option<f64>,
    pub interest_rate: Option<f64>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
}

impl UpdateLoan {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(amount) = self.amount {
            if amount <= 0.0 {
                return Err("Amount must be greater than zero".to_string());
            }
        }
        if let Some(rate) = self.interest_rate {
            if rate < 0.0 || rate > 1.0 {
                return Err("Interest rate must be between 0 and 1".to_string());
            }
        }
        if let Some(due_date) = self.due_date {
            let today = Utc::now().date_naive();
            if due_date <= today {
                return Err("Due date must be in the future".to_string());
            }
        }
        Ok(())
    }
}