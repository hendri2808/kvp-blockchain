CREATE TABLE reward_pools (
    pool_id SERIAL PRIMARY KEY,
    pool_name VARCHAR(50),
    total_tokens NUMERIC(20, 8) DEFAULT 0,
    allocated_tokens NUMERIC(20, 8) DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
