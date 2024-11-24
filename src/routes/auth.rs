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
 use serde::{Deserialize, Serialize};
 use bcrypt::{hash, verify};
 use crate::models::{User, KYCStatus, UserRole};
 use chrono::Utc;
 use uuid::Uuid;
 use std::collections::HashMap;
 use warp::reply::{json, with_status};
 
 pub type Db = Arc<Mutex<Vec<User>>>;
 
 // Struktur untuk permintaan Register
 #[derive(Debug, Serialize, Deserialize)]
 struct RegisterRequest {
     username: String,
     email: String,
     password: String,
 }
 
 // Struktur untuk permintaan Login
 #[derive(Debug, Serialize, Deserialize)]
 struct LoginRequest {
     username_or_email: String,
     password: String,
 }
 
 // Struktur untuk respons Login
 #[derive(Debug, Serialize, Deserialize)]
 struct LoginResponse {
     token: String,
     kyc_status: KYCStatus,
 }
 
 pub fn user_routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
     let register = warp::path("register")
         .and(warp::post())
         .and(warp::body::json())
         .and(with_db(db.clone()))
         .and_then(register_user);
 
     let login = warp::path("login")
         .and(warp::post())
         .and(warp::body::json())
         .and(with_db(db.clone()))
         .and_then(login_user);
 
     register.or(login)
 }
 
 // Fungsi untuk mendaftarkan user baru
 async fn register_user(body: RegisterRequest, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let mut db = db.lock().unwrap();

    // Periksa apakah username atau email sudah digunakan
    if db.iter().any(|u| u.username == body.username || u.email == body.email) {
        let error_response = json(&HashMap::from([("error", "User already exists")]));
        return Ok(with_status(error_response, StatusCode::BAD_REQUEST));
    }

    // Hash password menggunakan bcrypt
    let hashed_password = hash(&body.password, 10).unwrap(); // Gunakan referensi di sini
    let user_id = Uuid::new_v4().to_string();

    // Buat user baru
    let user = User {
        id: user_id.clone(),
        username: body.username.clone(), // Clone untuk menghindari move
        email: body.email.clone(),       // Clone untuk menghindari move
        password_hash: hashed_password,
        is_verified: false,
        kyc_status: KYCStatus::Pending,
        created_at: Utc::now().timestamp_millis() as u64,
        updated_at: Utc::now().timestamp_millis() as u64,
        role: UserRole::User,
    };

    // Masukkan user ke database
    db.push(user);

    // Log data yang diterima
    println!("Register request body: {:?}", body); // Gunakan `body` dengan aman

    // Respons berhasil
    let success_response = json(&HashMap::from([
        ("message", "User registered successfully"),
        ("user_id", user_id.as_str()),
    ]));
    Ok(with_status(success_response, StatusCode::OK))
}
 
 // Fungsi untuk login user
 async fn login_user(body: LoginRequest, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let db = db.lock().unwrap();

    // Cari user berdasarkan username atau email
    let user = db.iter().find(|u| {
        u.username == body.username_or_email || u.email == body.username_or_email
    });

    if let Some(user) = user {
        // Verifikasi password
        if verify(&body.password, &user.password_hash).unwrap() {
            let token = Uuid::new_v4().to_string(); // Dummy token
            let success_response = json(&HashMap::from([
                ("token", token),
                ("kyc_status", user.kyc_status.to_string()),
                ("user_id", user.id.clone())
            ]));
            println!("Login request body: {:?}", body);
            return Ok(with_status(success_response, StatusCode::OK));
        }
    }

    // Respons error jika login gagal
    let error_response = json(&HashMap::from([("error", "Invalid credentials")]));
    println!("Login failed for: {:?}", body);
    Ok(with_status(error_response, StatusCode::UNAUTHORIZED))
}

  // Helper untuk memberikan akses ke database
 fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
     warp::any().map(move || db.clone())
 }