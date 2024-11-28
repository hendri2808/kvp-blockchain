CREATE TABLE marketplace (
    listing_id SERIAL PRIMARY KEY, -- ID unik untuk listing
    nft_id INT REFERENCES nfts(nft_id) ON DELETE CASCADE, -- Foreign key ke tabel nfts
    seller_wallet TEXT NOT NULL, -- Wallet penjual
    buyer_wallet TEXT, -- Wallet pembeli (nullable jika belum terjual)
    price NUMERIC(20, 8) NOT NULL, -- Harga NFT
    status VARCHAR(20) DEFAULT 'Available', -- Status listing (Available, Sold, Cancelled)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Waktu listing dibuat
);
