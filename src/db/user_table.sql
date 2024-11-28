CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,           -- ID unik user
    username VARCHAR(50) NOT NULL UNIQUE, -- Username user
    email VARCHAR(100) NOT NULL UNIQUE,   -- Email user
    password_hash TEXT NOT NULL,          -- Password terenkripsi
    kyc_status VARCHAR(20) DEFAULT 'Pending', -- Status KYC (Pending, Approved)
    total_balance NUMERIC(20, 8) DEFAULT 0,   -- Total saldo user
    registration_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Waktu registrasi
    mining_status VARCHAR(10) DEFAULT NULL CHECK (mining_status IN ('Mobile', 'Node')), -- Status Mining
    utility_status JSONB DEFAULT '{}'::JSONB, -- Status utilitas lain (Game, DeFi, NFT, dll)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Waktu pembuatan data
);
