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

 use warp::{http::StatusCode, Filter};
 use std::sync::{Arc, Mutex};
 use std::collections::HashMap;
 use serde::{Deserialize};
 use crate::models::{KYCStatus, Db};
 use crate::middleware::auth_middleware::admin_only_with_db;
 
 #[derive(Debug, Deserialize)]
 struct KycStatusUpdateRequest {
     user_id: String,
     status: String, // "approved" or "rejected"
 }
 
 #[derive(Debug, Deserialize)]
 struct RewardRequest {
     amount: u64,
     operation: String, // "add" or "subtract"
 }
 
 pub fn admin_routes(db: Db, reward_pool: Arc<Mutex<u64>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let approve_kyc = warp::path!("admin" / "kyc")
        .and(warp::post())
        .and(admin_only_with_db(db.clone(), "my_secret"))
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(|_user_id: String, body: KycStatusUpdateRequest, db: Db| async move {
            approve_reject_kyc(body, db).await
        });

    let reward = warp::path!("admin" / "reward")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_reward_pool(reward_pool.clone()))
        .and_then(update_reward_pool);

    let stats = warp::path!("admin" / "stats")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(with_reward_pool(reward_pool.clone()))
        .and_then(get_admin_stats);

    approve_kyc.or(reward).or(stats)
}
 
 // Fungsi untuk approve/reject KYC
 async fn approve_reject_kyc(body: KycStatusUpdateRequest, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db = db.lock().unwrap();

    println!("Database: {:?}", db);
    println!("Request: {:?}", body);

    if let Some(user) = db.iter_mut().find(|u| u.id == body.user_id) {
        match body.status.as_str() {
            "approved" => user.kyc_status = KYCStatus::Approved,
            "rejected" => user.kyc_status = KYCStatus::Rejected,
            _ => {
                return Ok(warp::reply::with_status(
                    "Invalid status".to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }
        }

        println!("KYC status updated for user: {}", body.user_id);
        return Ok(warp::reply::with_status(
            format!("KYC status updated to {}", body.status),
            StatusCode::OK,
        ));
    }

    println!("User ID not found in database: {}", body.user_id);
    Err(warp::reject::not_found())
 }

 async fn update_reward_pool(body: RewardRequest, reward_pool: Arc<Mutex<u64>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut pool = reward_pool.lock().unwrap();

    match body.operation.as_str() {
        "add" => *pool += body.amount,
        "subtract" => *pool = pool.saturating_sub(body.amount),
        _ => return Err(warp::reject::custom(AuthError)),
    }

    Ok(warp::reply::json(&HashMap::from([
        ("message", "Reward pool updated".to_string()),
        ("total_reward_pool", pool.to_string()),
    ])))
 }
 
 async fn get_admin_stats(db: Db, reward_pool: Arc<Mutex<u64>>) -> Result<impl warp::Reply, warp::Rejection> {
     let db = db.lock().unwrap();
     let pool = reward_pool.lock().unwrap();
 
     let total_users = db.len();
     let kyc_pending = db.iter().filter(|u| u.kyc_status == KYCStatus::Pending).count();
     let kyc_approved = db.iter().filter(|u| u.kyc_status == KYCStatus::Approved).count();
     let kyc_rejected = db.iter().filter(|u| u.kyc_status == KYCStatus::Rejected).count();
 
     let stats = HashMap::from([
         ("total_users", total_users.to_string()),
         ("kyc_pending", kyc_pending.to_string()),
         ("kyc_approved", kyc_approved.to_string()),
         ("kyc_rejected", kyc_rejected.to_string()),
         ("total_reward_pool", pool.to_string()),
     ]);
 
     Ok(warp::reply::json(&stats))
 }
 
 // Helper untuk mengakses database
 fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
     warp::any().map(move || db.clone())
 }
 
 // Helper untuk mengakses reward pool
 fn with_reward_pool(pool: Arc<Mutex<u64>>) -> impl Filter<Extract = (Arc<Mutex<u64>>,), Error = std::convert::Infallible> + Clone {
     warp::any().map(move || pool.clone())
 }
 
 // Custom error type for rejection
 #[derive(Debug)]
 struct AuthError;
 
 impl warp::reject::Reject for AuthError {}
 