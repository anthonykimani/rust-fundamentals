fn main() {
    let mut connection = database_connection();

    let newUser = users::NewUsers {
        name: String::from("Anto"),
        firstname: String::from("Kim"),
        age: 16u8,
        email: String::from("kimani@gmail.com")
    };

    let record = diesel::insert_into(schema::users::table).values(&newUser).get_result::<users::Users>(&mut connection).expect("Error saving new example");

    println!("{:?}", record);
}
pub fn database_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}