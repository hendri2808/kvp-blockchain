CREATE OR REPLACE FUNCTION verify_kyc_automatically()
RETURNS TRIGGER AS $$
DECLARE
    v_status VARCHAR;
BEGIN
    -- Cek apakah dokumen sudah di-upload dan apakah selfie sesuai dengan dokumen
    SELECT status INTO v_status
    FROM kyc_submissions
    WHERE user_id = NEW.user_id;  -- Menggunakan NEW untuk mendapatkan nilai user_id yang baru dimasukkan

    IF v_status = 'pending' THEN
        -- Verifikasi otomatis: Jika selfie cocok, status berubah menjadi 'approved'
        -- Misalnya kita anggap verifikasi sukses jika tidak ada error
        UPDATE kyc_submissions
        SET status = 'approved', updated_at = NOW()
        WHERE user_id = NEW.user_id;  -- Menggunakan NEW untuk mendapatkan nilai user_id

        -- Insert ke tabel kyc_approvals secara otomatis
        INSERT INTO kyc_approvals (user_id, status, created_at, updated_at)
        VALUES (NEW.user_id, 'approved', NOW(), NOW());

        -- Update status kyc_status pada users menjadi 'verified'
        UPDATE users 
        SET kyc_status = 'verified'
        WHERE user_id = NEW.user_id;  -- Menggunakan NEW untuk mendapatkan nilai user_id
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
