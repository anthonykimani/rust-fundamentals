-- Your SQL goes here
CREATE TABLE loan_applicants(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    loan_id INTEGER NOT NULL REFERENCES loans(id),
    application_status VARCHAR NOT NULL,
    application_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    approved_date TIMESTAMP
)
