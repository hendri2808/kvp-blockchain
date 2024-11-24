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

 use sha2::{Digest, Sha256};
 use serde::{Serialize, Deserialize};
 use chrono::Utc;
 use std::collections::HashMap;
 use warp::{Filter, Reply, cors};
 use tokio::sync::broadcast;
 use warp::ws::{Ws, WebSocket};
 use warp::ws::Message as WsMessage;
 use std::sync::{Arc, Mutex};
 use futures::{StreamExt, SinkExt}; 
 
 #[tokio::main]
 async fn main() {
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
 
     // Gabungkan semua route
     let routes = blockchain_route
     .or(stats_route)
     .or(block_detail_route)
     .or(search_route)
     .or(ws_route)
     .with(cors());
 
     println!("Server running at http://127.0.0.1:3030");
     warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
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
             Utc::now().timestamp_millis() as u128,
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
             Utc::now().timestamp_millis() as u128,
             transactions,
             last_block.hash.clone(),
         );
         self.chain.push(new_block);
 
         // Kirim update ke WebSocket
         tx.send(serde_json::to_string(&self.chain).unwrap()).unwrap();
     }
 }
 