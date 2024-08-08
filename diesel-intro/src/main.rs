mod models;
mod db_operations;
mod controllers;
mod schema;

use std::sync::Mutex;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_session::config::{ BrowserSession, CookieContentSecurity };
use actix_web::{cookie::{Key}, web, App, HttpServer, Responder,  middleware, HttpResponse};
use crate::controllers::users::{login_page, login_user, register_page, register_user};
use db_operations::db;
use actix_web::cookie::SameSite;
use dotenvy::dotenv;
use controllers::home::default_handler;
use crate::controllers::available_loans::{get_all_available_loans, get_available_loans_by_item};
use crate::controllers::loans::{create_loan, get_all_loans, get_loans_by_item, loan_page};
use crate::models::app_state::AppState;
use actix_cors::Cors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();


    HttpServer::new(move || {
        let app_state = web::Data::new(AppState {db_connection: Mutex::new(db::establish_connection())  });
        let secret_key = Key::generate();
        App::new()
            .app_data(app_state.clone())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name(String::from("user_client"))
                    .cookie_secure(false) // set to true if using HTTPS
                    .cookie_http_only(true)
                    .session_lifecycle(BrowserSession::default())
                    .cookie_same_site(SameSite::Lax)
                    // .cookie_content_security(CookieContentSecurity::Private)
                    .build()
            )
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:7000") // Allow requests from your frontend
                    .allowed_methods(vec!["GET", "POST"]) // Allow these HTTP methods
                    .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600)
            )
            // .route("/dashboard", web::get().to(dashboard_page))
            .route("/login", web::post().to(login_user))
            .route("/login", web::get().to(login_page))
            .route("/register", web::post().to(register_user))
            .route("/register", web::get().to(register_page))
            .route("/loan/create", web::post().to(create_loan))
            .route("/loans", web::get().to(get_all_loans))
            .route("/loan", web::get().to(loan_page))
            .route("/loan/{id}", web::get().to(get_loans_by_item))
            .route("/available_loans", web::get().to(get_all_available_loans))
            .route("/available_loans/{id}", web::get().to(get_available_loans_by_item))
            .default_service(web::to(default_handler))
    })
        .bind(("127.0.0.1", 7000))?.run().await
}
