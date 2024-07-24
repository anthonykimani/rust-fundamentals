-- Your SQL goes here
CREATE TABLE loan_repayments(
    id SERIAL PRIMARY KEY,
    application_id INTEGER NOT NULL REFERENCES loan_applicants(id),
    repayment_amount INTEGER NOT NULL,
    repayment_date TIMESTAMP,
    status VARCHAR NOT NULL
)