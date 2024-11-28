CREATE TABLE transactions (
    transaction_id SERIAL PRIMARY KEY,
    transaction_hash TEXT NOT NULL UNIQUE,
    block_id INTEGER REFERENCES blocks(block_id) ON DELETE CASCADE,
    sender TEXT,
    receiver TEXT,
    amount NUMERIC(20, 8),
    signature TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
