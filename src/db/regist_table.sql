CREATE TABLE user_registrations (
    registration_id SERIAL PRIMARY KEY,   -- ID unik untuk registrasi
    user_id INT REFERENCES users(user_id) ON DELETE CASCADE, -- ID user
    ip_address VARCHAR(45),               -- IP Address saat registrasi
    device_info TEXT,                     -- Informasi perangkat
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- Waktu registrasi
);
