-- Your SQL goes here
CREATE TABLE loans(
    id SERIAL PRIMARY KEY,
    amount INTEGER NOT NULL,
    borrower_name TEXT NOT NULL,
    loan_date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id)
)