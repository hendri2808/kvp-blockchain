This document outlines the database structure for the KVP Blockchain project, covering all features and functionalities such as Core Blockchain Layer, User Management, Reward Pool, GameFi (Earn-to-Play), NFT Marketplace, and DAO governance.

---

## **1. Core Blockchain Layer**
### Tujuan:
- Menyimpan blok dan transaksi pada blockchain.
- Mendukung fitur mining dan validasi blok.

### Tabel:
1. **blocks**
   - id (Primary Key)
   - block_hash (Unique)
   - previous_hash
   - timestamp
   - data (JSONB)
   - nonce
   - created_at

2. **transactions**
   - id (Primary Key)
   - transaction_hash (Unique)
   - block_id (Foreign Key -> blocks.id)
   - sender (wallet_address)
   - receiver (wallet_address)
   - amount (DECIMAL)
   - signature (text)
   - created_at

---

## **2. User and Access Management**
### Tujuan:
- Mengelola pengguna (admin dan user).
- Menyediakan keamanan akses melalui KYC dan autentikasi.

### Tabel:
1. **admin**
   - admin_id (Primary Key)
   - username
   - password
   - created_at

2. **users**
   - user_id (Primary Key)
   - username
   - password
   - email
   - wallet_address (Unique)
   - created_at

3. **kyc_submissions**
   - kyc_id (Primary Key)
   - user_id (Foreign Key -> users.user_id)
   - document_type (e.g., ID Card, Passport)
   - document_path
   - status (Pending, Approved, Rejected)
   - submitted_at
   - reviewed_by (Foreign Key -> admin.admin_id, Nullable)
   - reviewed_at

4. **sessions**
   - session_id (Primary Key)
   - user_id (Foreign Key -> users.user_id)
   - token
   - created_at
   - expires_at

---

## **3. Reward Pool and Mining**
### Tujuan:
- Mengelola pool reward untuk mining dan GameFi.
- Mendukung pengelolaan reward secara otomatis.

### Tabel:
1. **reward_pools**
   - pool_id (Primary Key)
   - pool_name
   - total_tokens (DECIMAL)
   - allocated_tokens (DECIMAL)
   - created_at
   - updated_at

2. **mining_rewards**
   - reward_id (Primary Key)
   - miner_wallet (wallet_address)
   - block_id (Foreign Key -> blocks.id)
   - amount (DECIMAL)
   - created_at

3. **pool_allocations**
   - allocation_id (Primary Key)
   - pool_id (Foreign Key -> reward_pools.pool_id)
   - recipient_wallet (wallet_address)
   - allocation_type (e.g., Game, Staking, Mining)
   - amount (DECIMAL)
   - allocated_at

---

## **4. GameFi (Earn-to-Play)**
### Tujuan:
- Mendukung game berbasis blockchain.
- Memberikan reward kepada pemain.

### Tabel:
1. **games**
   - game_id (Primary Key)
   - name
   - description
   - developer_wallet (wallet_address)
   - revenue_share (DECIMAL)
   - created_at
   - updated_at

2. **game_sessions**
   - session_id (Primary Key)
   - game_id (Foreign Key -> games.game_id)
   - player_wallet (wallet_address)
   - session_start
   - session_end
   - status
   - rewards_earned (DECIMAL)

3. **game_rewards**
   - reward_id (Primary Key)
   - game_id (Foreign Key -> games.game_id)
   - player_wallet (wallet_address)
   - reward_type (Token, NFT, Bonus)
   - reward_value (DECIMAL or metadata ID)
   - created_at

4. **leaderboards**
   - leaderboard_id (Primary Key)
   - game_id (Foreign Key -> games.game_id)
   - player_wallet (wallet_address)
   - score
   - rank
   - updated_at

---

## **5. NFT and Marketplace**
### Tujuan:
- Mendukung marketplace untuk transaksi NFT.
- Mengintegrasikan NFT dengan GameFi dan DeFi.

### Tabel:
1. **nfts**
   - nft_id (Primary Key)
   - owner_wallet (wallet_address)
   - game_id (Foreign Key -> games.game_id, Nullable)
   - metadata (JSONB)
   - created_at

2. **marketplace**
   - listing_id (Primary Key)
   - nft_id (Foreign Key -> nfts.nft_id)
   - seller_wallet (wallet_address)
   - buyer_wallet (wallet_address, Nullable)
   - price (DECIMAL)
   - status
   - created_at

---

## **6. Governance (DAO)**
### Tujuan:
- Memberikan kekuatan voting kepada komunitas.
- Mengelola pengambilan keputusan secara transparan.

### Tabel:
1. **proposals**
   - proposal_id (Primary Key)
   - title
   - description
   - creator_wallet (wallet_address)
   - status
   - created_at
   - updated_at

2. **votes**
   - vote_id (Primary Key)
   - proposal_id (Foreign Key -> proposals.proposal_id)
   - voter_wallet (wallet_address)
   - vote_option (Yes, No, Abstain)
   - timestamp

---

## **Rencana Fondasi Database**
Berdasarkan struktur besar ini, kita akan membangun **fondasi database** dalam langkah-langkah berikut:

### **1. Fondasi Core Blockchain**
- Tabel: `blocks`, `transactions`.

### **2. Fondasi User Management**
- Tabel: `admin`, `users`, `kyc_submissions`, `sessions`.

### **3. Fondasi Reward Pool**
- Tabel: `reward_pools`, `mining_rewards`.

### **4. Fondasi GameFi**
- Tabel: `games`, `game_sessions`, `game_rewards`.

### **5. Fondasi NFT Marketplace**
- Tabel: `nfts`, `marketplace`.

### **6. Fondasi DAO**
- Tabel: `proposals`, `votes`.

---

## **Langkah Selanjutnya** - (Question on 26 Nov 2024) -
1. **Diskusi dan Validasi**:
   - Apakah struktur ini mencakup semua kebutuhan sistem KVP?
   - Apakah ada fitur tambahan yang perlu dipertimbangkan?

2. **Implementasi Bertahap**:
   - Pilih modul yang ingin diimplementasikan lebih dulu.
   - Mulai dari fondasi (Blockchain + User Management).

3. **Integrasi dan Pengujian**:
   - Pastikan semua modul bekerja secara terintegrasi.
   - Uji setiap fitur dengan skenario pengguna.