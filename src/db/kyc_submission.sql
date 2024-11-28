CREATE TABLE kyc_submissions (
    kyc_id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(user_id) ON DELETE CASCADE,
    document_path TEXT,
    document_type VARCHAR(10),
    status VARCHAR(20) DEFAULT 'Pending',
    submitted_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
