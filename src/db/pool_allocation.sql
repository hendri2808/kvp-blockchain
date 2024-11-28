CREATE TABLE pool_allocations (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    pool_id INT NOT NULL,
    allocated_amount DECIMAL(18, 8) NOT NULL,
    remaining_amount DECIMAL(18, 8) DEFAULT 0.0,
    timestamp TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (pool_id) REFERENCES reward_pools(pool_id) ON DELETE CASCADE
);
