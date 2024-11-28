CREATE TABLE votes (
    vote_id SERIAL PRIMARY KEY, -- ID unik untuk vote
    proposal_id INT REFERENCES proposals(proposal_id) ON DELETE CASCADE, -- Foreign key ke tabel proposals
    voter_wallet TEXT NOT NULL, -- Wallet voter
    vote_option VARCHAR(20) NOT NULL CHECK (vote_option IN ('Yes', 'No', 'Abstain')), -- Pilihan voting
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Waktu vote dilakukan
);
