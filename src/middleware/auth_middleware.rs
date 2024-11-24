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

 use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
 use warp::{Filter, Rejection};
 use serde::{Deserialize, Serialize};
 use crate::models::{Db, UserRole}; // Import model database dan enum role
 use crate::utils::time::current_time_in_wib;
 
 // Struktur payload JWT
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Claims {
     pub sub: String, // User ID
     pub exp: usize,  // Expiration timestamp
 }
 
 // Custom error untuk token tidak valid
 #[derive(Debug)]
 pub struct Unauthorized;
 
 impl warp::reject::Reject for Unauthorized {}
 
 // Middleware untuk validasi role admin
 pub fn admin_only_with_db(db: Db, secret: &'static str) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
     jwt_auth(secret).and_then(move |user_id: String| {
         let db = db.clone();
         async move {
             let db = db.lock().unwrap();
             if let Some(_user) = db.iter().find(|u| u.id == user_id && u.role == UserRole::Admin) {
                 Ok(user_id)
             } else {
                 Err(warp::reject::custom(Unauthorized))
             }
         }
     })
 }
 
 // Middleware untuk validasi token JWT
 pub fn jwt_auth(secret: &'static str) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
     warp::header::<String>("authorization")
         .and_then(move |token: String| {
             let secret = secret.to_string();
             async move {
                 match validate_jwt(&token, &secret) {
                     Ok(user_id) => Ok(user_id),
                     Err(_) => {
                         println!("[{}] Invalid JWT Token", current_time_in_wib().timestamp());
                         Err(warp::reject::custom(Unauthorized))
                     }
                 }
             }
         })
 }
 
 // Fungsi untuk memvalidasi token JWT
 fn validate_jwt(token: &str, secret: &str) -> Result<String, ()> {
     let token = token.trim_start_matches("Bearer "); // Hapus prefix "Bearer " jika ada
     let key = DecodingKey::from_secret(secret.as_ref());
     let validation = Validation::new(Algorithm::HS256);
 
     decode::<Claims>(token, &key, &validation)
         .map(|data| data.claims.sub) // Ambil user_id dari klaim
         .map_err(|_| ()) // Kembalikan error kosong untuk disesuaikan dengan tipe Result
 }
 
 #[allow(dead_code)]
 pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if let Some(_) = err.find::<crate::middleware::auth_middleware::Unauthorized>() {
        Ok(warp::reply::with_status(
            "Unauthorized access",
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
