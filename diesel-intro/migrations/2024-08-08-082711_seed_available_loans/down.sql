-- This file should undo anything in `up.sql`
DELETE FROM available_loans
WHERE (loan_type = 'microloan' AND amount = 1000 AND interest_rate = '5%' AND term_length = 12 AND description = 'Loan for small business expansion')
   OR (loan_type = 'personal loan' AND amount = 5000 AND interest_rate = '7%' AND term_length = 24 AND description = 'Loan for personal expenses')
   OR (loan_type = 'education loan' AND amount = 2000 AND interest_rate = '4%' AND term_length = 36 AND description = 'Loan for education purposes');
