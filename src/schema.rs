diesel::table! {
    loans (id) {
        id -> Int8,
        amount -> Float8,
        interest_rate -> Float8,
        due_date -> Timestamp,
        status -> VarChar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

#[derive(Insertable, Queryable, Identifiable, Associations, Debug, Clone)]
#[diesel(table_name = loans)]
#[diesel(belongs_to(LoanStatus))]
pub struct Loan {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: chrono::NaiveDateTime,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(QueryableByName, Debug, Clone)]
#[diesel(table_name = loans)]
pub struct LoanRow {
    pub id: i64,
    pub amount: f64,
    pub interest_rate: f64,
    pub due_date: chrono::NaiveDateTime,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl Loan {
    pub fn from_row(row: LoanRow) -> Self {
        Loan {
            id: row.id,
            amount: row.amount,
            interest_rate: row.interest_rate,
            due_date: row.due_date,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}