-- Remove indexes added for performance improvements
DROP INDEX IF EXISTS idx_cases_created_at;
DROP INDEX IF EXISTS idx_cases_status;
DROP INDEX IF EXISTS idx_transactions_user_id;
DROP INDEX IF EXISTS idx_investments_user_id;
DROP INDEX IF EXISTS idx_investments_case_id;
