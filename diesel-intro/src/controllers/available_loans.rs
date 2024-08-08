use actix_web::{HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use chrono::Utc;
use crate::db_operations::available_loans::get_available_loans_list;
use crate::models::app_state::AppState;
use crate::models::available_loans::AvailableLoan;
use crate::models::ui::LoginTemplate;

pub async fn get_all_available_loans(state: web::Data<AppState>) -> impl Responder {
    let mut connection_guard = state.db_connection.lock().unwrap();
    let available_loans = get_available_loans_list(&mut *connection_guard);

    let response = serde_json::to_string(&(*available_loans)).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}

pub async fn get_available_loans_by_item(state: web::Data<AppState>) -> impl Responder {
    log::info!("fetch all loans to implement");

    let now = Utc::now().naive_utc();
    let available_loan = AvailableLoan{
        id:0,
        loan_type: "test loan type".to_string(),
        amount: 0,
        interest_rate: "test interest rate".to_string(),
        term_length: 0,
        description: Some("".to_string()),
        created_at: now,
        updated_at: now
    };
    HttpResponse::Ok().json(&available_loan)
}