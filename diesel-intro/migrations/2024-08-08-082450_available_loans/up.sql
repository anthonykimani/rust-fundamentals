CREATE TABLE available_loans(
    id SERIAL PRIMARY KEY,
    loan_type VARCHAR NOT NULL,
    amount INTEGER NOT NULL,
    interest_rate VARCHAR NOT NULL,
    term_length INTEGER NOT NULL,
    description TEXT NULL,
    created_at timestamp without time zone default CURRENT_TIMESTAMP not null,
    updated_at timestamp without time zone default CURRENT_TIMESTAMP not null
)