CREATE TABLE game_sessions (
    session_id SERIAL PRIMARY KEY, -- ID unik untuk sesi game
    game_id INT REFERENCES games(game_id) ON DELETE CASCADE, -- Foreign key ke tabel games
    player_wallet TEXT NOT NULL, -- Wallet pemain
    session_start TIMESTAMP NOT NULL, -- Waktu sesi dimulai
    session_end TIMESTAMP, -- Waktu sesi berakhir
    status VARCHAR(20) DEFAULT 'Active', -- Status sesi
    rewards_earned NUMERIC(20, 8) DEFAULT 0 -- Total reward yang didapatkan
);
