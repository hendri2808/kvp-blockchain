CREATE TABLE kyc_approvals (
    approval_id SERIAL PRIMARY KEY,
    kyc_id INT REFERENCES kyc_submissions(kyc_id) ON DELETE CASCADE,
    admin_id INT REFERENCES admin(admin_id),
    status VARCHAR(20),
    remarks TEXT,
    approved_at TIMESTAMP
);
