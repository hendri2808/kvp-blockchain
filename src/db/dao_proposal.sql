CREATE TABLE proposals (
    proposal_id SERIAL PRIMARY KEY, -- ID unik untuk proposal
    title TEXT NOT NULL, -- Judul proposal
    description TEXT NOT NULL, -- Deskripsi proposal
    creator_wallet TEXT NOT NULL, -- Wallet pembuat proposal
    status VARCHAR(20) DEFAULT 'Open', -- Status proposal (Open, Closed, Approved, Rejected)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Waktu proposal dibuat
    updated_at TIMESTAMP -- Waktu proposal diperbarui
);
