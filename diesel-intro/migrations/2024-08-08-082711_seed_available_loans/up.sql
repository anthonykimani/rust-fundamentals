-- Your SQL goes here
INSERT INTO available_loans (loan_type, amount, interest_rate, term_length, description)
VALUES
    ('microloan', 1000, '5%', 12, 'Loan for small business expansion'),
    ('personal loan', 5000, '7%', 24, 'Loan for personal expenses'),
    ('education loan', 2000, '4%', 36, 'Loan for education purposes');
