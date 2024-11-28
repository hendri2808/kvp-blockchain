CREATE TABLE nfts (
    nft_id SERIAL PRIMARY KEY, -- ID unik untuk NFT
    owner_wallet TEXT NOT NULL, -- Wallet pemilik NFT
    metadata JSONB NOT NULL, -- Metadata NFT dalam format JSON
    game_id INT REFERENCES games(game_id) ON DELETE SET NULL, -- Foreign key ke game (opsional)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Waktu NFT dibuat
);
