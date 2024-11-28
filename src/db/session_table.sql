CREATE TABLE sessions (
    session_id SERIAL PRIMARY KEY,         -- ID unik untuk sesi login
    user_id INT REFERENCES users(user_id) ON DELETE CASCADE, -- ID user
    token TEXT NOT NULL,                   -- Token login
    ip_address VARCHAR(45),                -- IP Address saat login
    device_info TEXT,                      -- Informasi perangkat
    login_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Waktu login
    logout_time TIMESTAMP DEFAULT NULL     -- Waktu logout
);
