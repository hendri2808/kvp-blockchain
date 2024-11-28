CREATE TABLE IF NOT EXISTS blocks (
    block_id SERIAL PRIMARY KEY,           -- ID unik untuk block
    block_hash VARCHAR(64) NOT NULL UNIQUE, -- Hash unik untuk block
    previous_hash VARCHAR(64),             -- Hash dari block sebelumnya
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Waktu block terbentuk
    data JSONB,                            -- Data transaksi dalam block
    nonce INTEGER,                         -- Nilai nonce untuk mining
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Waktu dibuatnya block
    total_blocks INT NOT NULL,             -- Total blocks di jaringan
    active_blocks INT NOT NULL,            -- Total block aktif
    inactive_blocks INT NOT NULL,          -- Total block tidak aktif
    burned_blocks INT DEFAULT 0,           -- Total block yang dibakar
    remaining_blocks INT NOT NULL,         -- Sisa block yang bisa digunakan
    halving_schedule TIMESTAMP,            -- Jadwal halving
    last_block_time TIMESTAMP,             -- Waktu pembuatan block terakhir
    next_halving_time TIMESTAMP,           -- Waktu halving berikutnya
    last_burn_time TIMESTAMP              -- Waktu blok terakhir dibakar
);

-- Menambahkan kolom untuk infomasi burned blocks di tabel blocks
ALTER TABLE blocks
ADD COLUMN burned BOOLEAN DEFAULT FALSE,
ADD COLUMN burned_block_ids TEXT[];  -- Untuk menyimpan ID blok yang dibakar (opsional)

CREATE OR REPLACE FUNCTION update_block_stats() 
RETURNS TRIGGER AS $$
BEGIN
    -- Update statistik blok di tabel blocks
    UPDATE blocks
    SET total_blocks = total_blocks + 1, 
        active_blocks = active_blocks + 1, 
        remaining_blocks = remaining_blocks - 1,
        last_block_time = CURRENT_TIMESTAMP
    WHERE block_id = NEW.block_id;

    -- Halving - jika block_id mencapai kelipatan 1000
    IF NEW.block_id % 1000 = 0 THEN
        -- Jadwalkan halving untuk blok tersebut
        UPDATE blocks 
        SET halving_schedule = CURRENT_TIMESTAMP
        WHERE block_id = NEW.block_id;
    END IF;

    -- Burned Blocks - Menandakan blok yang dibakar
    IF NEW.burned = TRUE THEN
        -- Tambahkan logika jika blok dibakar
        UPDATE blocks 
        SET burned_block_ids = array_append(burned_block_ids, NEW.block_id::TEXT)
        WHERE block_id = NEW.block_id;

        -- Update statistik burned blocks
        UPDATE blocks
        SET burned_blocks = burned_blocks + 1
        WHERE block_id = NEW.block_id;
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_block_stats
AFTER INSERT ON blocks
FOR EACH ROW
EXECUTE FUNCTION update_block_stats();
