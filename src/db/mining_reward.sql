CREATE TABLE mining_rewards (
    reward_id SERIAL PRIMARY KEY,
    pool_id INT REFERENCES reward_pools(pool_id),
    miner_wallet VARCHAR(100),
    mining_type VARCHAR(10) CHECK (mining_type IN ('Mobile', 'Node')),
    reward_amount NUMERIC(20, 8),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
