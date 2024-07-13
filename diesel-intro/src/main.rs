mod models;

mod schema;
use std::sync::Mutex;
use actix_web::{ web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use schema::users::dsl::*;
use crate::schema::loans::dsl::loans;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User{
    id: i32,
    name:String,
    firstname: String,
    age: i32,
    email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Credentials{
    name:String,
    email:String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Loan{
    amount: i32,
    borrower_name: String,
    loan_date: String,
    due_date: String,
    user_id: i32
}


#[derive(Debug)]
struct AppState{
    users: Mutex<Vec<User>>,
    loans: Mutex<Vec<Loan>>
}

async fn get_all_users(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&state.users)
}

async fn add_users(user: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
    let mut all_users = state.users.lock().unwrap();
    all_users.push(user.into_inner());
    println!("Created {} Person", all_users.len());
    HttpResponse::Ok().finish()
}

async fn add_loan(loan: web::Json<Loan>, state: web::Data<AppState>) -> impl Responder {
    let mut all_loans = state.loans.lock().unwrap();
    all_loans.push(loan.into_inner());
    println!("Created {} Loan", all_loans.len());
    HttpResponse::Ok()
}

async fn get_loans(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&state.loans)
}

async fn get_loan_by_id(id_num:web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
    let all_loans = state.loans.lock().unwrap();
    let loan_id: i32 = *id_num;
    let loan = all_loans.iter().find(|loan| loan.user_id == loan_id);
    match loan {
        Some(loan) => HttpResponse::Ok().json(loan),
        None => HttpResponse::NotFound().finish(),
    }
}

async fn login(state: web::Data<AppState>, credentials: web::Json<Credentials>) -> impl Responder {
    let all_users = state.users.lock().unwrap();
    let user_found = all_users.iter().find(|user_element|user_element.firstname == credentials.name && user_element.email == credentials.email);
    match user_found {
        Some(user_found) => HttpResponse::Ok().json(user_found),
        None => HttpResponse::NotFound().finish(),
    }

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState{users: Mutex::new(Vec::new()), loans: Mutex::new(Vec::new()) });
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/get_all_users", web::get().to(get_all_users))
            .route("/add_users", web::post().to(add_users))
            .route("/add_loans", web::post().to(add_loan))
            .route("/get_loans", web::get().to(get_loans))
            .route("/get_loan_by_id/{id}", web::get().to(get_loan_by_id))
            .route("/login", web::post().to(login))
    })
        .bind(("127.0.0.1", 7000))?.run().await
}
