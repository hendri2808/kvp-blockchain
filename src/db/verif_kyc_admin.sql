CREATE OR REPLACE FUNCTION admin_approve_kyc(user_id INT, approval_status VARCHAR)
RETURNS VOID AS $$
BEGIN
    -- Update status di kyc_submissions berdasarkan approval dari admin
    UPDATE kyc_submissions
    SET status = approval_status, updated_at = NOW()
    WHERE user_id = user_id;

    -- Insert ke tabel kyc_approvals setelah approval/rejection dari admin
    INSERT INTO kyc_approvals (user_id, status, created_at, updated_at)
    VALUES (user_id, approval_status, NOW(), NOW());

    -- Update status kyc_status pada users sesuai dengan hasil approval
    IF approval_status = 'approved' THEN
        UPDATE users 
        SET kyc_status = 'verified'
        WHERE user_id = user_id;
    ELSE
        UPDATE users 
        SET kyc_status = 'rejected'
        WHERE user_id = user_id;
    END IF;
END;
$$ LANGUAGE plpgsql;
