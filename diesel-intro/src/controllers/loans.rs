use actix_web::{web, Responder, HttpResponse, HttpRequest, error::ErrorUnauthorized,};
use askama::Template;
use crate::db_operations::loans::add_loan;
use crate::models::ui::{DashboardTemplate, LoanTemplate, LoginTemplate};
use crate::models::app_state::AppState;
use crate::models::loans::{CreateLoanForm, Loan, NewLoan};
use crate::db_operations::users::{add_user, get_a_user_by_id, get_a_user_by_mail};
use actix_session::Session;
use chrono::Utc;
use log::{error, info};

async fn handle_loan_error(error: &str) -> HttpResponse {
    let template = LoanTemplate { error: None, message: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

async fn handle_loan_information(error: &str) -> HttpResponse {
    let template = LoanTemplate { error: None, message: Some(error.to_string()) };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}
pub async fn create_loan(loan: web::Form<CreateLoanForm>, state: web::Data<AppState>, session: Session) -> HttpResponse {
    println!("Data is {:#?}", loan);
    if loan.loan_type.is_empty() || loan.amount.eq(&0) || loan.term_length.eq(&0) {
        println!("Empty fields detected");
        return handle_loan_error("All fields are required").await;
    }

    println!("All fields have content");

    // let user_email = session.get::<String>("user_email");

    let mut connection_guard = state.db_connection.lock().unwrap();
    let result = match session.get::<String>("user_email") {
        Ok(Some(user_email)) => {
            info!("user_id found in session: {}", user_email);
            match get_a_user_by_mail(&mut connection_guard, user_email) {
                Some(user) => {
                    let dashboard_template = DashboardTemplate {
                        email: user.email.to_string(),
                    };
                    println!("User found");
                    Ok(HttpResponse::Ok().content_type("text/html").body(dashboard_template.render().unwrap()))
                }
                None => {
                    println!("User not found");
                    Ok(HttpResponse::NotFound().append_header((actix_web::http::header::LOCATION, "/login")).finish())
                }
            }
        },
        Ok(None) => {
            info!("No user_id found in session");
            Ok(HttpResponse::Found()
                .append_header((actix_web::http::header::LOCATION, "/login"))
                .finish())
        },
        Err(e) => {
            error!("Session error: {:?}", e);
            Err(actix_web::error::ErrorInternalServerError("Session error"))
        }
    };
    let new_loan = NewLoan {
        loan_type: loan.loan_type.clone(),
        amount: loan.amount.clone(),
        interest_rate: loan.interest_rate.clone(),
        term_length: loan.term_length.clone(),
        description: Some("".to_string()),
        user_id: 1,
    };

    let res = add_loan(new_loan, &mut *connection_guard);

    match res {
        Ok(loan) => {
            println!("Loan Data: {:#?}", loan);
            return handle_loan_information("Loan created, please login to continue").await;
        }
        Err(err) => {
            println!("db error {:#?}", err);
            return handle_loan_error("error creating loan").await;
        }
    }
}

pub async fn loan_page(error: Option<String>, message: Option<String>) -> impl Responder {
    let template = LoanTemplate { error, message };
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}

pub async fn get_all_loans(state: web::Data<AppState>) -> impl Responder {
    log::info!("fetch all loans to implement");
    let v :Vec<Loan> = Vec::new();
    HttpResponse::Ok().json(&v)
}

pub async fn get_loans_by_item(state: web::Data<AppState>) -> impl Responder {
    log::info!("fetch all loans to implement");

    let now = Utc::now().naive_utc();
    let loan = Loan{
        id:0,
        loan_type: "test loan type".to_string(),
        amount: 0,
        interest_rate: "test interest rate".to_string(),
        term_length: 0,
        description: Some("".to_string()),
        user_id: 0,
        created_at: now,
        updated_at: now
    };
    HttpResponse::Ok().json(&loan)
}

pub async fn update_post(item: web::Json<Loan>, state: web::Data<AppState>) -> impl Responder {
    log::info!("to implment");
    HttpResponse::Ok().finish()
}

pub async fn delete_post_item(path: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    // HttpResponse::Ok().finish()
    log::info!("Register user to implment");
    HttpResponse::NotFound().finish()
}
