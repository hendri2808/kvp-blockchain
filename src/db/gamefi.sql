CREATE TABLE games (
    game_id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    developer_wallet VARCHAR(100),
    revenue_share NUMERIC(20, 8),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
