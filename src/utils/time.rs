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

 use chrono::{DateTime, Utc};
 use chrono_tz::Asia::Jakarta;
 
 /// Mendapatkan waktu sekarang dalam zona WIB (UTC+7).
 pub fn current_time_in_wib() -> DateTime<chrono_tz::Tz> {
     let utc_now: DateTime<Utc> = Utc::now(); // Waktu sekarang di UTC
     utc_now.with_timezone(&Jakarta) // Konversi ke WIB
 }
 