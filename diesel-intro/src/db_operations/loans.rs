use diesel::prelude::*;
use crate::models::loans::{Loan, NewLoan};

pub fn get_all_loans(connection: &mut PgConnection) -> Vec<Loan> {
    use crate::schema::loans::dsl::*;

    let mut all_loans: Vec<Loan> = Vec::new();
    let results = loans
        .select(Loan::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for loan in data.into_iter() {
                all_loans.push(loan)
            }

            println!("todo")
        }
        Err(e) => println!("Error occured {:?}", e)
    }

    return all_loans;
}

pub fn get_loan_by_id(connection: &mut PgConnection, loan_id: i32) -> Option<Loan> {
    use crate::schema::loans::dsl::*;


    loans
        .filter(id.eq(loan_id))
        .select(Loan::as_select())
        .first::<Loan>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}


pub fn add_loan(new_loan: NewLoan, connection: &mut PgConnection) -> Result<Loan, diesel::result::Error>{
    diesel::insert_into(crate::schema::loans::table)
        .values(&new_loan)
        .returning(Loan::as_returning())
        .get_result::<Loan>(connection)
}