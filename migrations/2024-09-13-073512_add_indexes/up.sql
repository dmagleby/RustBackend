-- Your SQL goes here
-- Add indexes to improve performance on frequently queried columns
CREATE INDEX idx_cases_created_at ON cases (created_at);
CREATE INDEX idx_cases_status ON cases (status);
CREATE INDEX idx_transactions_user_id ON transactions (user_id);
CREATE INDEX idx_investments_user_id ON investments (user_id);
CREATE INDEX idx_investments_case_id ON investments (case_id);
