This document outlines the database structure for the KVP Blockchain project, covering all features and functionalities such as Core Blockchain Layer, User Management, Reward Pool, GameFi (Earn-to-Play), NFT Marketplace, and DAO governance.

---

# **DATA POSTGRESQL kvp_blockchain**

## 1. kvp_blockchain=# -- Main Table

### kvp_blockchain=# \d+ blocks;
                                                                          Table "public.blocks"
                                                                          
|      Column       |            Type             | Collation | Nullable |                 Default                  | Storage  | Compression | Stats target | Description |
|-------------------|-----------------------------|-----------|----------|------------------------------------------|----------|-------------|--------------|-------------|
| block_id          | integer                     |           | not null | nextval('blocks_block_id_seq'::regclass) | plain    |             |         		| 			        |
| block_hash        | text                        |           | not null |                                          | extended |             |         		| 			        |
| previous_hash     | text                        |           |          |                                          | extended |             |         		| 			  |
| timestamp         | timestamp without time zone |           |          | CURRENT_TIMESTAMP                        | plain    |             |         		| 			  |
| data              | jsonb                       |           |          |                                          | extended |             |         		| 			  |
| nonce             | integer                     |           |          |                                          | plain    |             |         		| 			  |
| created_at        | timestamp without time zone |           |          | CURRENT_TIMESTAMP                        | plain    |             |         		| 			  |
| transactions      | jsonb                       |           |          |                                          | extended |             |         		| 			  |
| total_blocks      | integer                     |           | not null | 0                                        | plain    |             |         		| 			  |
| active_blocks     | integer                     |           | not null | 0                                        | plain    |             |         		| 			  |
| inactive_blocks   | integer                     |           | not null | 0                                        | plain    |             |         		| 			  |
| burned_blocks     | integer                     |           |          | 0                                        | plain    |             |         		| 			  |
| remaining_blocks  | integer                     |           | not null | 0                                        | plain    |             |         		| 			  |
| halving_schedule  | timestamp without time zone |           |          |                                          | plain    |             |         		| 			  |
| last_block_time   | timestamp without time zone |           |          |                                          | plain    |             |         		| 			  |
| next_halving_time | timestamp without time zone |           |          |                                          | plain    |             |         		| 			  |
| last_burn_time    | timestamp without time zone |           |          |                                          | plain    |             |         		| 			  |
| burned            | boolean                     |           |          | false                                    | plain    |             |         		| 			  |
| burned_block_ids  | text[]                      |           |          |                                          | extended |             |         		| 			  |
```
Indexes:

    "blocks_pkey" PRIMARY KEY, btree (block_id)
    "blocks_block_hash_key" UNIQUE CONSTRAINT, btree (block_hash)
    
Referenced by:

    TABLE "transactions" CONSTRAINT "fk_block_id" FOREIGN KEY (block_id) REFERENCES blocks(block_id) ON DELETE CASCADE
    TABLE "mining_rewards" CONSTRAINT "mining_rewards_block_id_fkey" FOREIGN KEY (block_id) REFERENCES blocks(block_id) ON DELETE CASCADE
    TABLE "transactions" CONSTRAINT "transactions_block_id_fkey" FOREIGN KEY (block_id) REFERENCES blocks(block_id) ON DELETE CASCADE
    
Triggers:

    trigger_update_block_stats AFTER INSERT ON blocks FOR EACH ROW EXECUTE FUNCTION update_block_stats()
    
Access method: heap
```

### kvp_blockchain=# \d+ transactions;
                                                                            Table "public.transactions"
																			
|      Column      |            Type             | Collation | Nullable |                       Default      				   | Storage  | Compression | Stats target | Description |
|------------------|-----------------------------|-----------|----------|------------------------------------------------------|----------|-------------|--------------|-------------|
| transaction_id   | integer                     |           | not null | nextval('transactions_transaction_id_seq'::regclass) | plain    |             |              |			 |
| transaction_hash | text                        |           | not null |												       | extended |             |              |			 |
| block_id         | integer                     |           |          |												       | plain    |             |              |			 |
| sender           | text                        |           |          |												       | extended |             |              |			 |
| receiver         | text                        |           |          |												       | extended |             |              |			 |
| amount           | numeric(20,8)               |           |          |												       | main     |             |              |			 |
| signature        | text                        |           |          |												       | extended |             |              |			 |
| created_at       | timestamp without time zone |           |          | CURRENT_TIMESTAMP								       | plain    |             |              |			 |
| user_id          | integer                     |           |          |												       | plain    |             |              |			 |
| sender_id        | integer                     |           |          |												       | plain    |             |              |			 |
| receiver_id      | integer                     |           |          |												       | plain    |             |              |			 |
```
Indexes:

    "transactions_pkey" PRIMARY KEY, btree (transaction_id)
    "idx_block_id" btree (block_id)
    "idx_user_id" btree (user_id)
    "transactions_transaction_hash_key" UNIQUE CONSTRAINT, btree (transaction_hash)

Foreign-key constraints:

    "fk_block_id" FOREIGN KEY (block_id) REFERENCES blocks(block_id) ON DELETE CASCADE
    "fk_receiver" FOREIGN KEY (receiver_id) REFERENCES users(user_id) ON DELETE CASCADE
    "fk_receiver_id" FOREIGN KEY (receiver_id) REFERENCES users(user_id) ON DELETE SET NULL
    "fk_sender" FOREIGN KEY (sender_id) REFERENCES users(user_id) ON DELETE CASCADE
    "fk_sender_id" FOREIGN KEY (sender_id) REFERENCES users(user_id) ON DELETE SET NULL
    "fk_user_id" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    "transactions_block_id_fkey" FOREIGN KEY (block_id) REFERENCES blocks(block_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain=# \d+ admin;
                                                                        Table "public.admin"
																		
|    Column     |            Type             | Collation | Nullable |                 Default                 | Storage  | Compression | Stats target | Description |
|---------------|-----------------------------|-----------|----------|-----------------------------------------|----------|-------------|--------------|-------------|
| admin_id      | integer                     |           | not null | nextval('admin_admin_id_seq'::regclass) | plain    |             |              |			 |
| username      | character varying(50)       |           | not null |                                         | extended |             |              |			 |
| password_hash | text                        |           | not null |                                         | extended |             |              |			 |
| email         | character varying(100)      |           | not null |                                         | extended |             |              |			 |
| role          | character varying(20)       |           |          | 'admin'::character varying              | extended |             |              |			 |
| created_at    | timestamp without time zone |           |          | CURRENT_TIMESTAMP                       | plain    |             |              |			 |
```
Indexes:

    "admin_pkey" PRIMARY KEY, btree (admin_id)
    "admin_email_key" UNIQUE CONSTRAINT, btree (email)
    "admin_username_key" UNIQUE CONSTRAINT, btree (username)
	
Referenced by:

    TABLE "kyc_submissions" CONSTRAINT "fk_reviewed_by_admin" FOREIGN KEY (reviewed_by) REFERENCES admin(admin_id) ON DELETE SET NULL
    TABLE "kyc_approvals" CONSTRAINT "kyc_approvals_admin_id_fkey" FOREIGN KEY (admin_id) REFERENCES admin(admin_id)
	
Access method: heap
```

### kvp_blockchain=# \d+ user_registrations;
                                                                            Table "public.user_registrations"
																			
|     Column      |            Type             | Collation | Nullable |                           Default				             | Storage  | Compression | Stats target | Description |
|-----------------|-----------------------------|-----------|----------|-------------------------------------------------------------|----------|-------------|--------------|-------------|
| registration_id | integer                     |           | not null | nextval('user_registrations_registration_id_seq'::regclass) | plain    |             |              |			   |
| user_id         | integer                     |           |          |													         | plain    |             |              |			   |	
| ip_address      | character varying(45)       |           |          |													         | extended |             |              |			   |
| device_info     | text                        |           |          |													         | extended |             |              |			   |
| timestamp       | timestamp without time zone |           |          | CURRENT_TIMESTAMP								             | plain    |             |              |			   |
```
Indexes:

    "user_registrations_pkey" PRIMARY KEY, btree (registration_id)

Foreign-key constraints:

    "fk_user_registration" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    "user_registrations_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain=# kvp_blockchain=# \d+ users
                                                                         Table "public.users"
																		 
|      Column       |            Type             | Collation | Nullable |                Default                 | Storage  | Compression | Stats target | Description |
|-------------------|-----------------------------|-----------|----------|----------------------------------------|----------|-------------|--------------|-------------|
| user_id           | integer                     |           | not null | nextval('users_user_id_seq'::regclass) | plain    |             |              |				|
| username          | character varying(50)       |           | not null |                                        | extended |             |              |				|
| email             | character varying(100)      |           | not null |                                        | extended |             |              |				|
| password_hash     | text                        |           | not null |                                        | extended |             |              |				|
| kyc_status        | character varying(20)       |           |          | 'Pending'::character varying           | extended |             |              |				|
| total_balance     | numeric(20,8)               |           |          | 0                                      | main  	 |             |              |				|
| registration_time | timestamp without time zone |           |          | CURRENT_TIMESTAMP                      | plain  	 |             |              |				|
| mining_status     | character varying(10)       |           |          | NULL::character varying                | extended |             |              |				|
| utility_status    | jsonb                       |           |          | '{}'::jsonb                            | extended |             |              |				|
| created_at        | timestamp without time zone |           |          | CURRENT_TIMESTAMP                      | plain  	 |             |              |				|
| additional_data   | jsonb                       |           |          |                                        | extended |             |              |				|
| updated_at        | timestamp without time zone |           | not null | CURRENT_TIMESTAMP                      | plain  	 |             |              |				|
```
Indexes:

    "users_pkey" PRIMARY KEY, btree (user_id)
    "users_email_key" UNIQUE CONSTRAINT, btree (email)
    "users_username_key" UNIQUE CONSTRAINT, btree (username)

Check constraints:

    "users_mining_status_check" CHECK (mining_status::text = ANY (ARRAY['Mobile'::character varying, 'Node'::character varying]::text[]))
	
Referenced by:

    TABLE "bandwidth" CONSTRAINT "bandwidth_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    TABLE "channel" CONSTRAINT "channel_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    TABLE "defi" CONSTRAINT "defi_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "blocks" CONSTRAINT "fk_miner_id" FOREIGN KEY (miner_id) REFERENCES users(user_id) ON DELETE SET NULL
    TABLE "transactions" CONSTRAINT "fk_receiver" FOREIGN KEY (receiver_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "transactions" CONSTRAINT "fk_receiver_id" FOREIGN KEY (receiver_id) REFERENCES users(user_id) ON DELETE SET NULL
    TABLE "transactions" CONSTRAINT "fk_sender" FOREIGN KEY (sender_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "transactions" CONSTRAINT "fk_sender_id" FOREIGN KEY (sender_id) REFERENCES users(user_id) ON DELETE SET NULL
    TABLE "transactions" CONSTRAINT "fk_user_id" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "kyc_approvals" CONSTRAINT "fk_user_id" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "mining_rewards" CONSTRAINT "fk_user_id" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "kyc_submissions" CONSTRAINT "fk_user_kyc" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "user_registrations" CONSTRAINT "fk_user_registration" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "sessions" CONSTRAINT "fk_user_session" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "gateway" CONSTRAINT "gateway_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    TABLE "kyc_submissions" CONSTRAINT "kyc_submissions_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "peer" CONSTRAINT "peer_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    TABLE "pool_allocations" CONSTRAINT "pool_allocations_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "seeder" CONSTRAINT "seeder_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    TABLE "sessions" CONSTRAINT "sessions_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    TABLE "user_registrations" CONSTRAINT "user_registrations_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
	
Access method: heap
```

### kvp_blockchain=# \d+ sessions;
                                                                        Table "public.sessions"
																		
|   Column    |            Type             | Collation | Nullable |                   Default                    | Storage  | Compression | Stats target | Description |
|-------------|-----------------------------|-----------|----------|----------------------------------------------|----------|-------------|--------------|-------------|
| session_id  | integer                     |           | not null | nextval('sessions_session_id_seq'::regclass) | plain    |             |              | |
| user_id     | integer                     |           |          |                                              | plain    |             |              | |
| token       | text                        |           | not null |                                              | extended |             |              | |
| ip_address  | character varying(45)       |           |          |                                              | extended |             |              | |
| device_info | text                        |           |          |                                              | extended |             |              | |
| login_time  | timestamp without time zone |           |          | CURRENT_TIMESTAMP                            | plain    |             |              | |
| logout_time | timestamp without time zone |           |          |                                              | plain    |             |              | |
```
Indexes:

    "sessions_pkey" PRIMARY KEY, btree (session_id)
	
Foreign-key constraints:

    "fk_user_session" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    "sessions_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
	
Access method: heap
```

### kvp_blockchain=# \d+ kyc_submissions;
                                                                       Table "public.kyc_submissions"
																	   
|    Column     |            Type             | Collation | Nullable |                     Default                     |Storage   | Compression | Stats target | Description |
|---------------|-----------------------------|-----------|----------|-------------------------------------------------|----------|-------------|--------------|-------------|
| kyc_id        | integer                     |           | not null | nextval('kyc_submissions_kyc_id_seq'::regclass) |plain     |             |              |			 |
| user_id       | integer                     |           |          |                                                 |plain     |             |              |			 |
| document_path | text                        |           |          |                                                 |extended  |             |              |			 |
| document_type | character varying(10)       |           |          |                                                 |extended  |             |              |			 |
| status        | character varying(20)       |           |          | 'Pending'::character varying                    |extended  |             |              |			 |
| submitted_at  | timestamp without time zone |           |          | CURRENT_TIMESTAMP                               |plain     |             |              |			 |
| reviewed_by   | integer                     |           |          |                                                 |plain     |             |              |			 |
| updated_at    | timestamp without time zone |           |          |                                                 |plain     |             |              |			 |
| created_at    | timestamp without time zone |           |          | now()                                           |plain     |             |              |			 |
```
Indexes:
    
	"kyc_submissions_pkey" PRIMARY KEY, btree (kyc_id)

Foreign-key constraints:

    "fk_reviewed_by_admin" FOREIGN KEY (reviewed_by) REFERENCES admin(admin_id) ON DELETE SET NULL
    "fk_user_kyc" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    "kyc_submissions_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
	
Referenced by:

    TABLE "kyc_approvals" CONSTRAINT "kyc_approvals_kyc_id_fkey" FOREIGN KEY (kyc_id) REFERENCES kyc_submissions(kyc_id) ON DELETE CASCADE
	
Triggers:

    trigger_kyc_submission AFTER INSERT ON kyc_submissions FOR EACH ROW EXECUTE FUNCTION verify_kyc_automatically()
	
Access method: heap
```

### kvp_blockchain=# \d+ mining_rewards;
                                                                        Table "public.mining_rewards"
																		
|    Column    |            Type             | Collation | Nullable |                      Default                      | Storage  | Compression | Stats target | Description |
|--------------|-----------------------------|-----------|----------|---------------------------------------------------|----------|-------------|--------------|-------------|
| reward_id    | integer                     |           | not null | nextval('mining_rewards_reward_id_seq'::regclass) | plain    |             |              |			  |
| miner_wallet | text                        |           | not null |                                                   | extended |             |              |			  |
| block_id     | integer                     |           |          |                                                   | plain    |             |              |			  |
| pool_id      | integer                     |           |          |                                                   | plain    |             |              |			  |
| amount       | numeric(20,8)               |           | not null |                                                   | main     |             |              |			  |
| created_at   | timestamp without time zone |           |          | CURRENT_TIMESTAMP                                 | plain    |             |              |			  |
| device_type  | character varying(50)       |           | not null |                                                   | extended |             |              |			  |
| reward       | double precision            |           | not null |                                                   | plain    |             |              |			  |
| user_id      | integer                     |           |          |                                                   | plain    |             |              |			  |
```
Indexes:

    "mining_rewards_pkey" PRIMARY KEY, btree (reward_id)

Foreign-key constraints:

    "fk_pool_id" FOREIGN KEY (pool_id) REFERENCES reward_pools(pool_id) ON DELETE SET NULL
    "fk_user_id" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
    "mining_rewards_block_id_fkey" FOREIGN KEY (block_id) REFERENCES blocks(block_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain=# \d+ pool_allocations;
                                                                      Table "public.pool_allocations"
																	  
|      Column      |            Type             | Collation | Nullable |                   Default                    | Storage | Compression | Stats target | Description |
|------------------|-----------------------------|-----------|----------|----------------------------------------------|---------|-------------|--------------|-------------|
| id               | integer                     |           | not null | nextval('pool_allocations_id_seq'::regclass) | plain   |             |              |				|
| user_id          | integer                     |           | not null |                                              | plain   |             |              |				|
| pool_id          | integer                     |           | not null |                                              | plain   |             |              |				|
| allocated_amount | numeric(18,8)               |           | not null |                                              | main    |             |              |				|
| remaining_amount | numeric(18,8)               |           |          | 0.0                                          | main    |             |              |				|
| timestamp        | timestamp without time zone |           |          | now()                                        | plain   |             |              |				|
| total_reward     | numeric                     |           |          | 0                                            | main    |             |              |				|
| reward_amount    | numeric                     |           |          | 0                                            | main    |             |              |				|
```
Indexes:

    "pool_allocations_pkey" PRIMARY KEY, btree (id)

Foreign-key constraints:

    "pool_allocations_pool_id_fkey" FOREIGN KEY (pool_id) REFERENCES reward_pools(pool_id) ON DELETE CASCADE
    "pool_allocations_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain=# \d+ reward_pools;                                                                         
										Table "public.reward_pools"
																		 
|      Column      |            Type             | Collation | Nullable |                    Default                    | Storage  | Compression | Stats target | Description |
|------------------|-----------------------------|-----------|----------|-----------------------------------------------|----------|-------------|--------------|-------------|
| pool_id          | integer                     |           | not null | nextval('reward_pools_pool_id_seq'::regclass) | plain    |             |              |			  |
| pool_name        | character varying(50)       |           |          |                                               | extended |             |              |			  |
| total_tokens     | numeric(20,8)               |           |          | 0                                             | main     |             |              |			  |
| allocated_tokens | numeric(20,8)               |           |          | 0                                             | main     |             |              |			  |
| created_at       | timestamp without time zone |           |          | CURRENT_TIMESTAMP                             | plain    |             |              |			  |
```
Indexes:

    "reward_pools_pkey" PRIMARY KEY, btree (pool_id)

Referenced by:

    TABLE "mining_rewards" CONSTRAINT "fk_pool_id" FOREIGN KEY (pool_id) REFERENCES reward_pools(pool_id) ON DELETE SET NULL
    TABLE "pool_allocations" CONSTRAINT "pool_allocations_pool_id_fkey" FOREIGN KEY (pool_id) REFERENCES reward_pools(pool_id) ON DELETE CASCADE

Access method: heap
```

## 2. kvp_blockchain=# -- Utilities Table

### kvp_blockchain=# \d+ game_rewards;
                                                                        Table "public.game_rewards"
																		
|    Column     |            Type             | Collation | Nullable |                     Default                     | Storage  | Compression | Stats target | Description |
|---------------|-----------------------------|-----------|----------|-------------------------------------------------|----------|-------------|--------------|-------------|
| reward_id     | integer                     |           | not null | nextval('game_rewards_reward_id_seq'::regclass) | plain    |             |              | |
| game_id       | integer                     |           |          |                                                 | plain    |             |              | |
| player_wallet | text                        |           | not null |                                                 | extended |             |              | |
| reward_type   | character varying(50)       |           | not null |                                                 | extended |             |              | |
| reward_value  | numeric(20,8)               |           | not null |                                                 | main     |             |              | |
| created_at    | timestamp without time zone |           |          | CURRENT_TIMESTAMP                               | plain    |             |              | |
```
Indexes:

    "game_rewards_pkey" PRIMARY KEY, btree (reward_id)

Foreign-key constraints:

    "fk_game_id" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE
    "game_rewards_game_id_fkey" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain=# \d+ game_sessions;
                                                                         Table "public.game_sessions"
																		 
|     Column     |            Type             | Collation | Nullable |                      Default                      | Storage  | Compression | Stats target | Description |
|----------------|-----------------------------|-----------|----------|---------------------------------------------------|----------|-------------|--------------|-------------|
| session_id     | integer                     |           | not null | nextval('game_sessions_session_id_seq'::regclass) | plain    |             |              |		|
| game_id        | integer                     |           |          |                                                   | plain    |             |              |		|
| player_wallet  | text                        |           | not null |                                                   | extended |             |              |		|
| session_start  | timestamp without time zone |           | not null |                                                   | plain    |             |              |		|
| session_end    | timestamp without time zone |           |          |                                                   | plain    |             |              |		|
| status         | character varying(20)       |           |          | 'Active'::character varying                       | extended |             |              |		|
| rewards_earned | numeric(20,8)               |           |          | 0                                                 | main     |             |              |		|
```
Indexes:

    "game_sessions_pkey" PRIMARY KEY, btree (session_id)

Foreign-key constraints:

    "fk_game_id" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE
    "game_sessions_game_id_fkey" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain=# \d+ games;
                                                                         Table "public.games"
																		 
|      Column      |            Type             | Collation | Nullable |                Default                 | Storage  | Compression | Stats target | Description |
|------------------|-----------------------------|-----------|----------|----------------------------------------|----------|-------------|--------------|-------------|
| game_id          | integer                     |           | not null | nextval('games_game_id_seq'::regclass) | plain    |             |              | |
| name             | character varying(100)      |           |          |                                        | extended |             |              | |
| developer_wallet | character varying(100)      |           |          |                                        | extended |             |              | |
| revenue_share    | numeric(20,8)               |           |          |                                        | main |             |              | |
| created_at       | timestamp without time zone |           |          | CURRENT_TIMESTAMP                      | plain |             |              | |
```
Indexes:

    "games_pkey" PRIMARY KEY, btree (game_id)

Referenced by:

    TABLE "game_rewards" CONSTRAINT "fk_game_id" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE
    TABLE "game_sessions" CONSTRAINT "fk_game_id" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE
    TABLE "game_rewards" CONSTRAINT "game_rewards_game_id_fkey" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE
    TABLE "game_sessions" CONSTRAINT "game_sessions_game_id_fkey" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE CASCADE
    TABLE "nfts" CONSTRAINT "nfts_game_id_fkey" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE SET NULL

Access method: heap
```

### kvp_blockchain=# \d+ marketplace;
                                                                         Table "public.marketplace"
																		 
|    Column     |            Type             | Collation | Nullable |                     Default                     | Storage  | Compression | Stats target | Description |
|---------------|-----------------------------|-----------|----------|-------------------------------------------------|----------|-------------|--------------|-------------|
| listing_id    | integer                     |           | not null | nextval('marketplace_listing_id_seq'::regclass) | plain    |             |              |			 |
| nft_id        | integer                     |           |          |                                                 | plain    |             |              |			 |
| seller_wallet | text                        |           | not null |                                                 | extended |             |              |			 |
| buyer_wallet  | text                        |           |          |                                                 | extended |             |              |			 |
| price         | numeric(20,8)               |           | not null |                                                 | main     |             |              |			 |
| status        | character varying(20)       |           |          | 'Available'::character varying                  | extended |             |              |			 |
| created_at    | timestamp without time zone |           |          | CURRENT_TIMESTAMP                               | plain    |             |              |			 |
```
Indexes:

    "marketplace_pkey" PRIMARY KEY, btree (listing_id)
	
Foreign-key constraints:

    "fk_nft_id" FOREIGN KEY (nft_id) REFERENCES nfts(nft_id) ON DELETE CASCADE
    "marketplace_nft_id_fkey" FOREIGN KEY (nft_id) REFERENCES nfts(nft_id) ON DELETE CASCADE
	
Access method: heap
```

### kvp_blockchain=# \d+ nfts;
                                                                      Table "public.nfts"
																	  
|    Column    |            Type             | Collation | Nullable |               Default                | Storage  | Compression | Stats target | Description |
|--------------|-----------------------------|-----------|----------|--------------------------------------|----------|-------------|--------------|-------------|
| nft_id       | integer                     |           | not null | nextval('nfts_nft_id_seq'::regclass) | plain    |        	    |              |			 |
| owner_wallet | text                        |           | not null |                                      | extended |      	    |              |			 |
| metadata     | jsonb                       |           | not null |                                      | extended | 	 	    |              |			 |
| game_id      | integer                     |           |          |                                      | plain    |     	    |              |			 |
| created_at   | timestamp without time zone |           |          | CURRENT_TIMESTAMP                    | plain    |    	        |              |			 |
```
Indexes:

    "nfts_pkey" PRIMARY KEY, btree (nft_id)

Foreign-key constraints:

    "nfts_game_id_fkey" FOREIGN KEY (game_id) REFERENCES games(game_id) ON DELETE SET NULL

Referenced by:

    TABLE "marketplace" CONSTRAINT "fk_nft_id" FOREIGN KEY (nft_id) REFERENCES nfts(nft_id) ON DELETE CASCADE
    TABLE "marketplace" CONSTRAINT "marketplace_nft_id_fkey" FOREIGN KEY (nft_id) REFERENCES nfts(nft_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain=# \d+ bandwidth;
                                                                    Table "public.bandwidth"
																	
|     Column      |           Type           | Collation | Nullable |                Default                | Storage | Compression | Stats target | Description |
|-----------------|--------------------------|-----------|----------|---------------------------------------|---------|-------------|--------------|-------------|
| id              | integer                  |           | not null | nextval('bandwidth_id_seq'::regclass) | plain   |             |              |			 |
| user_id         | integer                  |           |          |                                       | plain   |             |              |			 |
| bandwidth_limit | integer                  |           | not null |                                       | plain   |             |              |			 |
| usage           | integer                  |           | not null |                                       | plain   |             |              |			 |
| created_at      | timestamp with time zone |           |          | CURRENT_TIMESTAMP                     | plain   |             |              |			 |
```
Indexes:

    "bandwidth_pkey" PRIMARY KEY, btree (id)

Foreign-key constraints:

    "bandwidth_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)

Triggers:

    update_bandwidth_usage_trigger AFTER INSERT ON bandwidth FOR EACH ROW EXECUTE FUNCTION update_bandwidth_usage()

Access method: heap
```

### kvp_blockchain=# \d+ cloud;
                                                                  Table "public.cloud"
																  
                                                                  
|   Column   |           Type           | Collation | Nullable |              Default              | Storage  | Compression | Stats target | Description |
|------------|--------------------------|-----------|----------|-----------------------------------|----------|-------------|--------------|-------------|
| id         | integer                  |           | not null | nextval('cloud_id_seq'::regclass) | plain    |             |              | 			 |
| file_path  | character varying(255)   |           | not null |                                   | extended |             |              | 			 |
| size       | bigint                   |           | not null |                                   | plain    |             |              | 			 |
| status     | character varying(50)    |           |          | 'active'::character varying       | extended |             |              | 			 |
| created_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                 | plain    |             |              | 			 |
| updated_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                 | plain    |             |              | 			 |
```
Indexes:

    "cloud_pkey" PRIMARY KEY, btree (id)
    
Referenced by:

    TABLE "mirror" CONSTRAINT "mirror_cloud_id_fkey" FOREIGN KEY (cloud_id) REFERENCES cloud(id) ON DELETE CASCADE
    TABLE "peer" CONSTRAINT "peer_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    TABLE "seeder" CONSTRAINT "seeder_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    
Access method: heap
```

### kvp_blockchain=# \d+ mirror;
                                                                  Table "public.mirror"
																  
                                                                  
|   Column    |           Type           | Collation | Nullable |              Default               | Storage  | Compression | Stats target | Description |
|-------------|--------------------------|-----------|----------|------------------------------------|----------|-------------|--------------|-------------|
| id          | integer                  |           | not null | nextval('mirror_id_seq'::regclass) | plain    |             |              | 			   |
| cloud_id    | integer                  |           |          |                                    | plain    |             |              | 			   |
| mirror_path | character varying(255)   |           | not null |                                    | extended |             |              | 			   |
| created_at  | timestamp with time zone |           |          | CURRENT_TIMESTAMP                  | plain    |             |              | 			   |
```
Indexes:

    "mirror_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "mirror_cloud_id_fkey" FOREIGN KEY (cloud_id) REFERENCES cloud(id) ON DELETE CASCADE
    
Access method: heap
```

### kvp_blockchain=# \d+ seeder;
                                                                  Table "public.seeder"
																  
                                                                  
|   Column   |           Type           | Collation | Nullable |              Default               | Storage  | Compression | Stats target | Description |
|------------|--------------------------|-----------|----------|------------------------------------|----------|-------------|--------------|-------------|
| id         | integer                  |           | not null | nextval('seeder_id_seq'::regclass) | plain    |             |              | 			      |
| user_id    | integer                  |           |          |                                    | plain    |             |              | 			      |
| file_id    | integer                  |           |          |                                    | plain    |             |              | 			      |
| status     | character varying(50)    |           |          | 'active'::character varying        | extended |             |              | 		    	  |
| created_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                  | plain    |             |              | 			      |
```
Indexes:

    "seeder_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "seeder_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    "seeder_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Access method: heap
```

### kvp_blockchain=# \d+ peer;
                                                                  Table "public.peer"
																  
                                                                  
|   Column   |           Type           | Collation | Nullable |             Default              | Storage  | Compression | Stats target | Description |
|------------|--------------------------|-----------|----------|----------------------------------|----------|-------------|--------------|-------------|
| id         | integer                  |           | not null | nextval('peer_id_seq'::regclass) | plain    |             |              | 			|
| user_id    | integer                  |           |          |                                  | plain    |             |              | 			|
| file_id    | integer                  |           |          |                                  | plain    |             |              | 			|
| status     | character varying(50)    |           |          | 'active'::character varying      | extended |             |              | 			|
| created_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                | plain    |             |              | 			|
```
Indexes:

    "peer_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "peer_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    "peer_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Triggers:

    peer_status_update AFTER UPDATE ON peer FOR EACH ROW EXECUTE FUNCTION update_peer_status()
    
Access method: heap
```

### kvp_blockchain=# \d+ tracker;
                                                                  Table "public.tracker"
																  
|   Column    |           Type           | Collation | Nullable |               Default               | Storage  | Compression | Stats target | Description |
|-------------|--------------------------|-----------|----------|-------------------------------------|----------|-------------|--------------|-------------|
| id          | integer                  |           | not null | nextval('tracker_id_seq'::regclass) | plain    |		       |              |				|
| cloud_id    | integer                  |           |          |                                     | plain    |		       |              |				|
| tracker_url | character varying(255)   |           | not null |                                     | extended |		       |              |				|
| status      | character varying(50)    |           |          | 'active'::character varying         | extended |		       |              |				|
| created_at  | timestamp with time zone |           |          | CURRENT_TIMESTAMP                   | plain    |		       |              |				|
```
Indexes:
 
   "tracker_pkey" PRIMARY KEY, btree (id)
   
Access method: heap
```

### kvp_blockchain=# \d+ channel;
                                                                   Table "public.channel"
																   
                                                                   
|    Column    |           Type           | Collation | Nullable |               Default               | Storage  | Compression | Stats target | Description |
|--------------|--------------------------|-----------|----------|-------------------------------------|----------|-------------|--------------|-------------|
| id           | integer                  |           | not null | nextval('channel_id_seq'::regclass) | plain    |             |              | 			 |
| user_id      | integer                  |           |          |                                     | plain    |             |              | 			 |
| access_point | character varying(255)   |           | not null |                                     | extended |             |              | 			 |
| created_at   | timestamp with time zone |           |          | CURRENT_TIMESTAMP                   | plain    |             |              | 			 |
```
Indexes:

    "channel_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "channel_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Access method: heap
```

### kvp_blockchain=# \d+ dns;
                                                                  Table "public.dns"
																  
                                                                  
|   Column    |           Type           | Collation | Nullable |             Default             | Storage  | Compression | Stats target | Description |
|-------------|--------------------------|-----------|----------|---------------------------------|----------|-------------|--------------|-------------|
| id          | integer                  |           | not null | nextval('dns_id_seq'::regclass) | plain    |             |              | 			|
| domain_name | character varying(255)   |           | not null |                                 | extended |             |              | 			|
| ip_address  | character varying(45)    |           |          |                                 | extended |             |              | 			|
| created_at  | timestamp with time zone |           |          | CURRENT_TIMESTAMP               | plain    |             |              | 			|
```
Indexes:

    "dns_pkey" PRIMARY KEY, btree (id)
    
Access method: heap
```

### kvp_blockchain=# \d+ bandwidth;
                                                                    Table "public.bandwidth"
                                                                    
|     Column      |           Type           | Collation | Nullable |                Default                | Storage | Compression | Stats target | Description |
|-----------------|--------------------------|-----------|----------|---------------------------------------|---------|-------------|--------------|-------------|
| id              | integer                  |           | not null | nextval('bandwidth_id_seq'::regclass) | plain   |             | 			   | 			 |
| user_id         | integer                  |           |          |                                       | plain   |             | 			   | 			 |
| bandwidth_limit | integer                  |           | not null |                                       | plain   |             | 			   | 			 |
| usage           | integer                  |           | not null |                                       | plain   |             | 			   | 			 |	
| created_at      | timestamp with time zone |           |          | CURRENT_TIMESTAMP                     | plain   |             | 			   | 			 |
```
Indexes:

    "bandwidth_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "bandwidth_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Triggers:

    update_bandwidth_usage_trigger AFTER INSERT ON bandwidth FOR EACH ROW EXECUTE FUNCTION update_bandwidth_usage()
    
Access method: heap
```

### kvp_blockchain=# \d+ gateway;
                                                                  Table "public.gateway"
                                                                  
|   Column    |           Type           | Collation | Nullable |               Default               | Storage  | Compression | Stats target | Description |
|-------------|--------------------------|-----------|----------|-------------------------------------|----------|-------------|--------------|-------------|
| id          | integer                  |           | not null | nextval('gateway_id_seq'::regclass) | plain    |             |              | 			|
| user_id     | integer                  |           |          |                                     | plain    |             |              | 			|
| share_point | character varying(255)   |           | not null |                                     | extended |             |              | 			|
| created_at  | timestamp with time zone |           |          | CURRENT_TIMESTAMP                   | plain    |             |              | 			|
```
Indexes:

    "gateway_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "gateway_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Triggers:

    update_gateway_share_trigger AFTER INSERT ON gateway FOR EACH ROW EXECUTE FUNCTION update_gateway_share()
    
Access method: heap
```

### kvp_blockchain=# \d+ proposals;
                                                                          Table "public.proposals"
																		  
|     Column     |            Type             | Collation | Nullable |                    Default                     | Storage  | Compression | Stats target | Description |
|----------------|-----------------------------|-----------|----------|------------------------------------------------|----------|-------------|--------------|-------------|
| proposal_id    | integer                     |           | not null | nextval('proposals_proposal_id_seq'::regclass) | plain    |             |              |			 |
| title          | text                        |           | not null |                                                | extended |             |              |			 |
| description    | text                        |           | not null |                                                | extended |             |              |			 |
| creator_wallet | text                        |           | not null |                                                | extended |             |              |			 |
| status         | character varying(20)       |           |          | 'Open'::character varying                      | extended |             |              |			 |
| created_at     | timestamp without time zone |           |          | CURRENT_TIMESTAMP                              | plain    |             |              |			 |
| updated_at     | timestamp without time zone |           |          |                                                | plain    |             |              |			 |
```
Indexes:

    "proposals_pkey" PRIMARY KEY, btree (proposal_id)

Referenced by:

    TABLE "votes" CONSTRAINT "fk_proposal_id" FOREIGN KEY (proposal_id) REFERENCES proposals(proposal_id) ON DELETE CASCADE
    TABLE "votes" CONSTRAINT "votes_proposal_id_fkey" FOREIGN KEY (proposal_id) REFERENCES proposals(proposal_id) ON DELETE CASCADE

Access method: heap
```

### kvp_blockchain-# kvp_blockchain=# \d+ votes
                                                                       Table "public.votes"
								       
|    Column    |            Type             | Collation | Nullable |                Default                 | Storage  | Compression | Stats target | Description |
|--------------|-----------------------------|-----------|----------|----------------------------------------|----------|-------------|--------------|-------------|
| vote_id      | integer                     |           | not null | nextval('votes_vote_id_seq'::regclass) | plain    |           |              | |
| proposal_id  | integer                     |           |          |                                        | plain    |           |              | |
| voter_wallet | text                        |           | not null |                                        | extended |           |              | |
| vote_option  | character varying(20)       |           | not null |                                        | extended |           |              | |
| timestamp    | timestamp without time zone |           |          | CURRENT_TIMESTAMP                      | plain    |           |              | |
 
```
Indexes:

    "votes_pkey" PRIMARY KEY, btree (vote_id)

Check constraints:

    "votes_vote_option_check" CHECK (vote_option::text = ANY (ARRAY['Yes'::character varying, 'No'::character varying, 'Abstain'::character varying]::text[]))

Foreign-key constraints:

    "fk_proposal_id" FOREIGN KEY (proposal_id) REFERENCES proposals(proposal_id) ON DELETE CASCADE
    "votes_proposal_id_fkey" FOREIGN KEY (proposal_id) REFERENCES proposals(proposal_id) ON DELETE CASCADE

Access method: heap
'''

## 3. kvp_blockchain=# -- Migration Table

### kvp_blockchain=# \d+ __diesel_schema_migrations;

                                               Table "public.__diesel_schema_migrations"

                                               
| Column  |            Type             | Collation | Nullable |      Default      | Storage  | Compression | Stats target | Description |
|---------|-----------------------------|-----------|----------|-------------------|----------|-------------|--------------|-------------|
| version | character varying(50)       |           | not null |                   | extended |             |              | 			 |
| run_on  | timestamp without time zone |           | not null | CURRENT_TIMESTAMP | plain    |             |              | 			 |

```
Indexes:

    "__diesel_schema_migrations_pkey" PRIMARY KEY, btree (version)
    
Access method: heap
```

---

# **Penyesuaian Struktur Tabel Menurut PostgreSQL 28 Nov 2024**
```
src/

├── blockchain/
│ ├── mod.rs
│ ├── block.rs
│ └── transaction.rs
├── auth/
│ ├── mod.rs
│ ├── admin.rs
│ ├── user.rs
│ └── session.rs
├── kyc/
│ ├── mod.rs
│ ├── kyc_approval.rs
│ └── kyc_submission.rs
├── utility/
│ ├── dao/
│ │ ├── mod.rs
│ │ ├── vote.rs
│ │ └── proposal.rs
│ ├── defi.rs
│ ├── gamefi.rs
│ ├── nft.rs
│ ├── marketplace/
│ │ ├── marketplace.rs
│ │ └── item.rs
│ ├── storage/
│ │ ├── cloud.rs
│ │ ├── mirror.rs
│ │ ├── peer.rs
│ │ ├── seeder.rs
│ │ └── tracker.rs
│ └── kvpnet/
│   ├── bandwith.rs
│   ├── channel.rs
│   ├── dns.rs
│   └── gateway.rs
├── mining/
│ └── mining.rs
├── pool/
│ └── pool.rs
├── middleware/  // Opsional (not yet exist)
│ └── middleware.rs // Opsional (not yet exist)
├── db/
│ ├── mod.rs
│ ├── connection.rs
│ └── schema.rs
└── main.rs
```
