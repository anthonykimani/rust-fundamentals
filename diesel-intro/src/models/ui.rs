
use actix_web::body;
use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate{
    pub(crate) error: Option<String>,
    pub(crate) message: Option<String>
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate{
    pub(crate) error: Option<String>
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct DashboardTemplate {
    pub(crate) email: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "loans.html")]
pub struct LoanTemplate{
    pub(crate) error: Option<String>,
    pub(crate) message: Option<String>
}