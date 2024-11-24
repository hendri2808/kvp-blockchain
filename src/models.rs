/*
 * KVP Blockchain - Created from scratch by Kraken (Hendri RH) and Bro CG
 * 
 * This project is a groundbreaking blockchain solution focused on:
 * - Lightweight and efficient design
 * - High-speed transactions
 * - Environmentally friendly and resource-optimized mechanisms
 * 
 * Built with Rust to ensure performance and reliability.
 * 
 * All rights reserved © 2024 Kraken & Bro CG
 */

 use serde::{Serialize, Deserialize};
 use std::sync::{Arc, Mutex};
 use std::fmt;

 /// Represents a user in the system.
 #[derive(Debug, Serialize, Deserialize, Clone)]
 pub struct User {
     /// Unique identifier for the user.
     pub id: String,
     /// Username of the user.
     pub username: String,
     /// Email address of the user.
     pub email: String,
     /// Hashed password for secure storage.
     pub password_hash: String,
     /// Indicates whether the user has completed KYC verification.
     pub is_verified: bool,
     /// Current KYC status of the user.
     pub kyc_status: KYCStatus,
     /// Timestamp when the user was created.
     pub created_at: u64,
     /// Timestamp of the last update to the user's data.
     pub updated_at: u64,
     /// Role of the user (e.g., Admin, User).
     pub role: UserRole,
 }
 
 /// Enum to represent KYC status for users.
 #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
 pub enum KYCStatus {
     Pending,
     Approved,
     Rejected,
 }
 
 impl fmt::Display for KYCStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KYCStatus::Pending => write!(f, "Pending"),
            KYCStatus::Approved => write!(f, "Approved"),
            KYCStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

 /// Represents a KYC submission for a user.
 #[derive(Debug, Serialize, Deserialize, Clone)]
 pub struct KYCSubmission {
     pub id: String, // UUID
     pub user_id: String, // Foreign Key
     pub document_type: String, // E.g., passport, id_card
     pub document_path: String, // File storage path
     pub status: KYCStatus,
     pub uploaded_at: i64, // Ubah tipe ini menjadi i64
 }

 #[derive(Debug, Serialize, Deserialize)]
 pub struct KycStatusUpdateRequest {
     pub user_id: String,
     pub status: String, // approved atau rejected
 }
 
 /// Enum to define the roles of users in the system.
 #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
 pub enum UserRole {
     /// Admin role with elevated permissions.
     Admin,
     /// Regular user role.
     User,
 }
 
 /// Type alias for a thread-safe database of users.
 pub type Db = Arc<Mutex<Vec<User>>>;
 
 