mod models;
mod db_operations;
mod controllers;
mod schema;

use std::sync::Mutex;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{cookie::{time::Duration, Key}, web, App, HttpServer, Responder,  middleware, HttpResponse};
use crate::controllers::users::{login_page, login_user, register_page, register_user};
use serde::{Deserialize, Serialize};
use db_operations::db;
use actix_session::config::PersistentSession;
use actix_web::cookie::SameSite;
use schema::users::dsl::*;
use crate::schema::loans::dsl::loans;
use dotenvy::dotenv;
use controllers::home::default_handler;
use crate::models::app_state::AppState;


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
                    .cookie_secure(false) // set to true if using HTTPS
                    .cookie_http_only(true)
                    .cookie_same_site(SameSite::Lax)
                    .build()
            )
            // .route("/get_all_users", web::get().to(get_all_users))
            // .route("/add_users", web::post().to(add_users))
            // .route("/add_loans", web::post().to(add_loan))
            // .route("/get_loans", web::get().to(get_loans))
            // .route("/get_loan_by_id/{id}", web::get().to(get_loan_by_id))
            .route("/login", web::post().to(login_user))
            .route("/login", web::get().to(login_page))
            .route("/register", web::post().to(register_user))
            .route("/register", web::get().to(register_page))
            .default_service(web::to(default_handler))
    })
        .bind(("127.0.0.1", 7000))?.run().await
}
