CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    amount NUMERIC NOT NULL,
    transaction_type VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cases (
    id SERIAL PRIMARY KEY,
    lawyer_id INTEGER NOT NULL REFERENCES users(id),
    title VARCHAR NOT NULL,
    description TEXT NOT NULL,
    funding_goal NUMERIC NOT NULL,
    current_funding NUMERIC NOT NULL DEFAULT 0,
    status VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE investments (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    case_id INTEGER NOT NULL REFERENCES cases(id),
    amount NUMERIC NOT NULL,
    investment_type VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);