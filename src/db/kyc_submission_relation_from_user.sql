CREATE OR REPLACE FUNCTION submit_kyc(user_id INT)
RETURNS VOID AS $$
BEGIN
    -- Insert ke kyc_submissions
    INSERT INTO kyc_submissions (user_id, status, created_at, updated_at)
    VALUES (user_id, 'pending', NOW(), NOW());
    
    -- Update status di tabel users menjadi 'pending'
    UPDATE users 
    SET kyc_status = 'pending'
    WHERE user_id = user_id;
END;
$$ LANGUAGE plpgsql;
