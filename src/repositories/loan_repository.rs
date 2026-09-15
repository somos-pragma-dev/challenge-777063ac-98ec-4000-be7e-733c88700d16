use crate::models::loan::{Loan, LoanStatus};
use crate::error::loan_error::LoanError;
use diesel::{prelude::*, PgConnection, QueryResult};
use std::sync::Arc;
use tokio::sync::RwLock;

pub trait LoanRepository: Send + Sync {
    fn create(&self, loan: Loan) -> Result<Loan, LoanError>;
    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError>;
    fn find_all(&self, page: usize, per_page: usize) -> Result<Vec<Loan>, LoanError>;
    fn update(&self, loan: Loan) -> Result<Loan, LoanError>;
    fn delete(&self, id: i64) -> Result<(), LoanError>;
    fn count(&self) -> Result<i64, LoanError>;
    fn find_by_status(&self, status: LoanStatus) -> Result<Vec<Loan>, LoanError>;
}

pub struct DieselLoanRepository {
    connection: Arc<RwLock<PgConnection>>,
}

impl DieselLoanRepository {
    pub fn new(connection: Arc<RwLock<PgConnection>>) -> Self {
        Self { connection }
    }
}

impl LoanRepository for DieselLoanRepository {
    fn create(&self, loan: Loan) -> Result<Loan, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let result = diesel::insert_into(loans::table)
            .values((
                loans::amount.eq(loan.amount),
                loans::interest_rate.eq(loan.interest_rate),
                loans::due_date.eq(loan.due_date),
                loans::status.eq(loan.as_str()),
            ))
            .get_result::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(result)
    }
    
    fn find_by_id(&self, id: i64) -> Result<Loan, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let result = loans::table
            .filter(loans::id.eq(id))
            .first::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(result)
    }
    
    fn find_all(&self, page: usize, per_page: usize) -> Result<Vec<Loan>, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let offset = (page.saturating_sub(1)) * per_page;
        
        let results = loans::table
            .order(loans::id.desc())
            .limit(per_page as i64)
            .offset(offset as i64)
            .load::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(results)
    }
    
    fn update(&self, loan: Loan) -> Result<Loan, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let result = diesel::update(loans::table.find(loan.id))
            .set((
                loans::amount.eq(loan.amount),
                loans::interest_rate.eq(loan.interest_rate),
                loans::due_date.eq(loan.due_date),
                loans::status.eq(loan.as_str()),
            ))
            .get_result::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(result)
    }
    
    fn delete(&self, id: i64) -> Result<(), LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        diesel::delete(loans::table.find(id))
            .execute(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(())
    }
    
    fn count(&self) -> Result<i64, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let count: i64 = loans::table
            .count()
            .get_result(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(count)
    }
    
    fn find_by_status(&self, status: LoanStatus) -> Result<Vec<Loan>, LoanError> {
        let conn = futures::executor::block_on(self.connection.try_write())
            .map_err(|e| LoanError::database(e.to_string()))?;
        
        use crate::schema::loans;
        
        let results = loans::table
            .filter(loans::status.eq(status.as_str()))
            .load::<Loan>(&*conn)
            .map_err(LoanError::from)?;
        
        Ok(results)
    }
}