use diesel::prelude::*;
use crate::models::available_loans::AvailableLoan;

pub fn get_available_loans_list(connection: &mut PgConnection) -> Vec<AvailableLoan> {
    use crate::schema::available_loans::dsl::*;

    let mut all_available_loans: Vec<AvailableLoan> = Vec::new();
    let results = available_loans
        .select(AvailableLoan::as_select())
        .load(connection);
    match results {
        Ok(data) => {
            for loan in data.into_iter() {
                all_available_loans.push(loan)
            }

            println!("todo")
        }
        Err(e) => println!("Error occured {:?}", e)
    }

    return all_available_loans;
}

pub fn get_available_loan_by_id(connection: &mut PgConnection, loan_id: i32) -> Option<AvailableLoan> {
    use crate::schema::available_loans::dsl::*;


    available_loans
        .filter(id.eq(loan_id))
        .select(AvailableLoan::as_select())
        .first::<AvailableLoan>(connection)
        .optional() // This will convert the result to Option
        .unwrap_or_else(|err| {
            println!("Error occurred: {:?}", err);
            None
        })
}