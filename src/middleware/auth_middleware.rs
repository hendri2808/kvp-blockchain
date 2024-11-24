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
 use warp::reject::{Reject};
 
 // Struktur payload JWT
 #[derive(Debug, Serialize, Deserialize)]
 pub struct Claims {
     pub sub: String, // User ID
     pub exp: usize,  // Expiration timestamp
 }
 
 // Custom error untuk token tidak valid
 #[derive(Debug)]
 struct Unauthorized;
 
 impl Reject for Unauthorized {}
 
 // Middleware untuk validasi token JWT
 pub fn jwt_auth(secret: &'static str) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
     warp::header::<String>("authorization")
         .and_then(move |token: String| {
             let secret = secret.to_string();
             async move {
                 match validate_jwt(&token, &secret) {
                     Ok(user_id) => Ok(user_id),
                     Err(_) => Err(warp::reject::custom(Unauthorized)),
                 }
             }
         })
 }
 
 // Fungsi untuk memvalidasi token JWT
 fn validate_jwt(token: &str, secret: &str) -> Result<String, ()> {
     let token = token.trim_start_matches("Bearer "); // Hapus prefix "Bearer " jika ada
     let key = DecodingKey::from_secret(secret.as_ref());
     let validation = Validation::new(Algorithm::HS256);
 
     match decode::<Claims>(token, &key, &validation) {
         Ok(data) => Ok(data.claims.sub), // Return `user_id`
         Err(_) => Err(()),              // Token invalid
     }
 }
 