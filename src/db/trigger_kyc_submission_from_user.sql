CREATE TRIGGER trigger_kyc_submission
AFTER INSERT ON kyc_submissions
FOR EACH ROW
EXECUTE FUNCTION verify_kyc_automatically();
