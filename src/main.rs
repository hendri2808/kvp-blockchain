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

 type UserDb = Arc<Mutex<Vec<User>>>;

 mod models;
 mod routes;
 mod middleware;
 mod utils;

 use sha2::{Digest, Sha256};
 use serde::{Serialize, Deserialize};
 use crate::utils::time::current_time_in_wib;
 use std::collections::HashMap;
 use warp::{Filter, Reply};
 use tokio::sync::broadcast;
 use warp::ws::{WebSocket};
 use warp::ws::Message as WsMessage;
 use std::sync::{Arc, Mutex};
 use futures::{StreamExt, SinkExt}; 
 use middleware::auth_middleware::jwt_auth;
 use routes::{auth::user_routes, kyc::kyc_routes, admin::admin_routes};
 use crate::models::User;
 use crate::routes::kyc::with_user_db;
 use warp::ws::Ws;

 // Database types
 type Db = Arc<Mutex<Vec<models::User>>>;
 type KycDb = Arc<Mutex<Vec<models::KYCSubmission>>>;  

 // Inisiaisasi semua Database Gabungan
 #[tokio::main]
 async fn main() {
     // Inisialisasi database pengguna
     let user_db: Db = Arc::new(Mutex::new(Vec::new()));

     let auth_secret = "my_secret";

     // Inisialisasi Dashboard
     let dashboard_route = warp::path("dashboard")
     .and(warp::get())
     .and(jwt_auth(auth_secret)) // Middleware untuk validasi token
     .and(with_user_db(user_db.clone()))
     .map(|user_id: String, user_db: UserDb| {
         let user_db = user_db.lock().unwrap();
         if let Some(user) = user_db.iter().find(|u| u.id == user_id) {
             warp::reply::json(user).into_response()
         } else {
             warp::reply::with_status("User not found", warp::http::StatusCode::NOT_FOUND).into_response()
         }
     }); 
 
     // Inisialisasi database untuk reward pool
     let reward_pool = Arc::new(Mutex::new(1000u64)); // Default 1000
 
     // Inisialisasi database KYC
     let kyc_db: KycDb = Arc::new(Mutex::new(Vec::new()));

     // Routes for user authentication and KYC
     let auth_routes = user_routes(user_db.clone());
     let kyc_routes = kyc_routes(kyc_db.clone(), user_db.clone());
     let admin_routes = admin_routes(user_db.clone(), reward_pool.clone());

     // Sampel untuk Protect Routes
     let _protected_kyc_routes = warp::path("protected_kyc")
         .and(jwt_auth(auth_secret))
         .and(warp::path::end())
         .map(|user_id: String| {
             format!("Access granted for user: {}", user_id)
     });
    
     // Setup channel untuk WebSocket broadcast
     let (tx, _rx) = broadcast::channel::<String>(100);
 
     // Setup Blockchain dan Network
     let mut blockchain = Blockchain::new();
     let mut network = Network::new(1000);
 
     // Inisialisasi Blockchain dan Network
     initialize_network(&mut blockchain, &mut network, &tx);
     println!("Total reward pool: {}", network.total_reward_pool);
 
     // Clone Blockchain untuk digunakan dalam route
     let blockchain_clone = blockchain.clone();
     let _blockchain_clone_stats = blockchain.clone();
     let _blockchain_clone_search = Arc::new(Mutex::new(blockchain.clone())); // Tetap atau hapus jika tidak digunakan
     let _blockchain_data = blockchain_clone.chain.clone(); // Tetap atau hapus jika tidak digunakan
     let blockchain_shared = Arc::new(Mutex::new(blockchain.clone()));
 
     // Route WebSocket
     let ws_route = warp::path("ws")
     .and(warp::ws())
     .and(warp::any().map(move || tx.clone()))
     .map(|ws: Ws, tx| {
         ws.on_upgrade(move |socket| handle_websocket(socket, tx))
     });
 
     // Route untuk mendapatkan semua blok
     let blockchain_route = warp::path("blockchain")
     .map({
         let blockchain_shared = Arc::clone(&blockchain_shared);
         move || {
             let blockchain_data = blockchain_shared.lock().unwrap();
             warp::reply::json(&blockchain_data.chain)
         }
     });

     // Route Untuk Block Detail
     let block_detail_route = warp::path!("block" / u64)
     .map({
         let blockchain_shared = Arc::clone(&blockchain_shared);
         move |index: u64| {
             let blockchain_data = blockchain_shared.lock().unwrap();
             let block = blockchain_data.chain.iter().find(|block| block.index == index);
             match block {
                 Some(block) => warp::reply::json(block).into_response(),
                 None => warp::reply::with_status("Block not found", warp::http::StatusCode::NOT_FOUND).into_response(),
             }
         }
     });

     // Route untuk mendapatkan statistik
     let stats_route = warp::path("stats")
     .map({
         let blockchain_shared = Arc::clone(&blockchain_shared);
         move || {
             let blockchain_data = blockchain_shared.lock().unwrap();
             let total_blocks = blockchain_data.chain.len();
             let total_transactions: usize = blockchain_data.chain.iter()
                 .map(|block| block.transactions.len())
                 .sum();
 
             let stats = HashMap::from([
                 ("total_blocks".to_string(), total_blocks.to_string()),
                 ("total_transactions".to_string(), total_transactions.to_string()),
                 ("reward_pool".to_string(), format!("{}", network.total_reward_pool)),
             ]);
 
             warp::reply::json(&stats)
         }
     }); 
 
     // Route untuk pencarian blok berdasarkan hash atau index
     let search_route = warp::path("search")
     .and(warp::filters::query::query::<HashMap<String, String>>())
     .map({
         let blockchain_shared = Arc::clone(&blockchain_shared);
         move |params: HashMap<String, String>| {
             // Perbaiki penggunaan temporary value
             let default_query = String::from(""); 
             let query = params.get("q").unwrap_or(&default_query);
             let blockchain_data = blockchain_shared.lock().unwrap();
             let results: Vec<Block> = blockchain_data.chain
                 .iter()
                 .filter(|block| block.hash.starts_with(query) || block.index.to_string().starts_with(query))
                 .cloned()
                 .collect();
             warp::reply::json(&results)
         }
     });

     // CORS Configuration
     let cors = warp::cors()
         .allow_any_origin()
         .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
         .allow_headers(vec!["Content-Type", "Authorization"]); // Tambahkan Authorization jika token digunakan
 
     // Gabungkan semua route
     let routes = auth_routes
     .or(dashboard_route)
     .or(kyc_routes)
     .or(admin_routes)
     .or(blockchain_route)
     .or(stats_route)
     .or(block_detail_route)
     .or(search_route)
     .or(ws_route)
     .with(cors);
 
     println!("Server running at http://127.0.0.1:3030");
     warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
 }

// Tambahkan fungsi handle_rejection jika belum ada
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

 // Inisialisasi Blockchain dan Network
 fn initialize_network(blockchain: &mut Blockchain, network: &mut Network, tx: &broadcast::Sender<String>) {
     // Daftarkan node
     network.register_node("Node1".to_string());
     network.register_node("Node2".to_string());
     network.register_node("Node3".to_string());
 
     // Update kontribusi
     network.update_contribution("Node1", 50);
     network.update_contribution("Node2", 30);
     network.update_contribution("Node3", 20);
 
     // Buat transaksi kontribusi
     let mut transactions = vec![];
     for (node_id, node) in &network.nodes {
         transactions.push(Transaction {
             sender: node_id.clone(),
             receiver: "Reward Pool".to_string(),
             amount: 0.0,
             contribution: node.contribution_score,
         });
     }
 
     // Tambahkan blok Genesis ke blockchain
     blockchain.add_block(transactions, tx);
 
     // Distribusi reward
     network.distribute_rewards();
 }
 
 // Handle WebSocket koneksi
 async fn handle_websocket(ws: WebSocket, tx: broadcast::Sender<String>) {
     let (mut ws_tx, mut ws_rx) = ws.split();
 
     // Kirimkan pesan sambutan
     if ws_tx.send(WsMessage::text("Listening for blockchain updates...")).await.is_err() {
         return;
     }
 
     // Subscribing dan menerima update
     let mut rx = tx.subscribe();
     tokio::spawn(async move {
         while let Ok(msg) = rx.recv().await {
             if ws_tx.send(WsMessage::text(msg)).await.is_err() {
                 break;
             }
         }
     });
 
     // Menerima pesan dari WebSocket
     while let Some(Ok(msg)) = ws_rx.next().await {
         if msg.is_text() || msg.is_binary() {
             println!("Received a message: {:?}", msg);
         }
     }
 }
 
 // Struktur Node
 #[derive(Debug, Clone)]
 struct Node {
     id: String,
     contribution_score: u64,
 }
 
 // Struktur Network
 struct Network {
     nodes: HashMap<String, Node>,
     total_reward_pool: u64,
 }
 
 impl Network {
     fn new(total_reward_pool: u64) -> Self {
         Self {
             nodes: HashMap::new(),
             total_reward_pool,
         }
     }
 
     fn register_node(&mut self, node_id: String) {
         let node = Node {
             id: node_id.clone(),
             contribution_score: 0,
         };
         self.nodes.insert(node_id, node);
     }
 
     fn update_contribution(&mut self, node_id: &str, score: u64) {
         if let Some(node) = self.nodes.get_mut(node_id) {
             node.contribution_score += score;
         }
     }
 
     fn distribute_rewards(&self) {
         let total_contribution: u64 = self.nodes.values().map(|node| node.contribution_score).sum();
 
         for node in self.nodes.values() {
             let reward = if total_contribution > 0 {
                 (node.contribution_score as f64 / total_contribution as f64 * self.total_reward_pool as f64) as u64
             } else {
                 0
             };
             println!("Node {} receives reward: {}", node.id, reward);
         }
     }
 }
 
 // Struktur Transaction
 #[derive(Serialize, Deserialize, Debug, Clone)]
 struct Transaction {
     sender: String,
     receiver: String,
     amount: f64,
     contribution: u64,
 }
 
 // Struktur Block
 #[derive(Serialize, Deserialize, Debug, Clone)]
 struct Block {
     index: u64,
     timestamp: u128,
     transactions: Vec<Transaction>,
     previous_hash: String,
     hash: String,
 }
 
 impl Block {
     fn new(index: u64, timestamp: u128, transactions: Vec<Transaction>, previous_hash: String) -> Self {
         let block_data = format!(
             "{}{}{:?}{}",
             index, timestamp, transactions, previous_hash
         );
         let hash = format!("{:x}", Sha256::digest(block_data.as_bytes()));
         Self {
             index,
             timestamp,
             transactions,
             previous_hash,
             hash,
         }
     }
 }
 
 // Struktur Blockchain
 #[derive(Clone)]
 struct Blockchain {
     chain: Vec<Block>,
 }
 
 impl Blockchain {
     fn new() -> Self {
         let genesis_transaction = Transaction {
             sender: "Genesis".to_string(),
             receiver: "Genesis".to_string(),
             amount: 0.0,
             contribution: 0,
         };
 
         let genesis_block = Block::new(
             0,
             current_time_in_wib().timestamp_millis() as u128, // WIB timestamp,
             vec![genesis_transaction],
             "0".to_string(),
         );
 
         Self {
             chain: vec![genesis_block],
         }
     }
 
     fn add_block(&mut self, transactions: Vec<Transaction>, tx: &broadcast::Sender<String>) {
         let last_block = self.chain.last().unwrap();
         let new_block = Block::new(
             last_block.index + 1,
             current_time_in_wib().timestamp_millis() as u128, // WIB timestamp,
             transactions,
             last_block.hash.clone(),
         );
         self.chain.push(new_block);
 
         // Kirim update ke WebSocket
         tx.send(serde_json::to_string(&self.chain).unwrap()).unwrap();
     }
 }
 