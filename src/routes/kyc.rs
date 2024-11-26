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
 use uuid::Uuid;
 use std::collections::HashMap;
 use crate::models::{KYCSubmission, KYCStatus, User};
 use crate::utils::time::current_time_in_wib;
 use warp::multipart::FormData;
 use tokio::{fs, io::AsyncWriteExt};
 use futures::TryStreamExt;
 use warp::Buf;
 
 type KycDb = Arc<Mutex<Vec<KYCSubmission>>>;
 type UserDb = Arc<Mutex<Vec<User>>>;
 
 #[derive(Debug, Serialize, Deserialize)]
 struct KycStatusUpdateRequest {
     user_id: String,
     status: String, // approved atau rejected
 }
 
 pub fn kyc_routes(
     kyc_db: KycDb,
     user_db: UserDb,
 ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
     let submit = warp::path("kyc")
         .and(warp::post())
         .and(warp::multipart::form())
         .and(with_kyc_db(kyc_db.clone()))
         .and_then(submit_kyc_data);
 
     let update_status = warp::path!("kyc" / "status")
         .and(warp::post())
         .and(warp::body::json())
         .and(with_kyc_db(kyc_db.clone()))
         .and(with_user_db(user_db.clone()))
         .and_then(update_kyc_status);
 
     submit.or(update_status)
 }
 
// Fungsi untuk menangani upload KYC
async fn submit_kyc_data(form: FormData, db: KycDb) -> Result<impl warp::Reply, warp::Rejection> {
    let doc_type = Arc::new(Mutex::new(String::new()));
    let user_id = Arc::new(Mutex::new(String::new()));
    let file_path = Arc::new(Mutex::new(String::new()));

    println!("Received multipart form data");

    // Menggunakan async move untuk closure di form.try_for_each
    form.try_for_each(|mut part| {
        let doc_type = Arc::clone(&doc_type);
        let user_id = Arc::clone(&user_id);
        let file_path = Arc::clone(&file_path);

        async move {
            match part.name() {
                "docType" => {
                    if let Some(Ok(data)) = part.data().await {
                        let bytes = data.chunk();
                        let mut doc_type = doc_type.lock().unwrap();
                        *doc_type = String::from_utf8_lossy(bytes).to_string();
                        println!("docType: {}", *doc_type);
                    }
                }
                "userId" => {
                    if let Some(Ok(data)) = part.data().await {
                        let bytes = data.chunk();
                        let mut user_id = user_id.lock().unwrap();
                        *user_id = String::from_utf8_lossy(bytes).to_string();
                        println!("userId: {}", *user_id);
                    }
                }
                "document" => {
                    if let Some(filename) = part.filename() {
                        let filepath = format!("./uploads/{}", filename);
                        let mut file = fs::File::create(&filepath).await.unwrap();

                        while let Some(Ok(chunk)) = part.data().await {
                            file.write_all(chunk.chunk()).await.unwrap();
                        }

                        let mut file_path = file_path.lock().unwrap();
                        *file_path = filepath;
                        println!("Document saved at: {}", *file_path);
                    }
                }
                _ => {}
            }
            Ok(())
        }
    })
    .await
    .unwrap();

    // Setelah data selesai diproses
    let user_id = Arc::try_unwrap(user_id).unwrap().into_inner().unwrap();
    let doc_type = Arc::try_unwrap(doc_type).unwrap().into_inner().unwrap();
    let file_path = Arc::try_unwrap(file_path).unwrap().into_inner().unwrap();

    let submission = KYCSubmission {
        id: Uuid::new_v4().to_string(),
        user_id,
        document_type: doc_type,
        document_path: file_path,
        status: KYCStatus::Pending,
        uploaded_at: current_time_in_wib().timestamp(),
    };

    let mut db = db.lock().unwrap();
    db.push(submission);

    let success_response = warp::reply::json(&HashMap::from([
        ("message", "KYC submitted successfully"),
    ]));
    Ok(warp::reply::with_status(success_response, StatusCode::OK))
}
 
 // Fungsi untuk mengubah status KYC
 async fn update_kyc_status(
     body: KycStatusUpdateRequest,
     kyc_db: KycDb,
     user_db: UserDb,
 ) -> Result<impl warp::Reply, warp::Rejection> {
     println!("Received KYC status update request: {:?}", body);
 
     let mut kyc_db = kyc_db.lock().unwrap();
 
     if let Some(submission) = kyc_db.iter_mut().find(|s| s.user_id == body.user_id) {
         match body.status.as_str() {
             "approved" => submission.status = KYCStatus::Approved,
             "rejected" => submission.status = KYCStatus::Rejected,
             _ => return Err(warp::reject::not_found()),
         }
 
         let mut user_db = user_db.lock().unwrap();
         if let Some(user) = user_db.iter_mut().find(|u| u.id == body.user_id) {
             user.kyc_status = submission.status.clone();
         }
 
         let success_response = warp::reply::json(&HashMap::from([
             ("message", format!("KYC status updated to {}", body.status)),
         ]));
         return Ok(warp::reply::with_status(success_response, StatusCode::OK));
     }
 
     Err(warp::reject::not_found())
 }
 
 // Helper untuk mengakses database KYC
 pub fn with_kyc_db(db: KycDb) -> impl Filter<Extract = (KycDb,), Error = std::convert::Infallible> + Clone {
     warp::any().map(move || db.clone())
 }
 
 // Helper untuk mengakses database User
 pub fn with_user_db(db: UserDb) -> impl Filter<Extract = (UserDb,), Error = std::convert::Infallible> + Clone {
     warp::any().map(move || db.clone())
 }
 