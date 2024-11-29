This document outlines the database structure for the KVP Blockchain project, covering all features and functionalities such as Core Blockchain Layer, User Management, Reward Pool, GameFi (Earn-to-Play), NFT Marketplace, and DAO governance.

---

## **DATA POSTGRESQL kvp_blockchain**
kvp_blockchain=# -- Menampilkan struktur tabel tertentu
kvp_blockchain=# \d+ transactions;
                                                                            Table "public.transactions"
                                                                            
|      Column      |            Type             | Collation | Nullable |                       Default                        | Storage  | Compression | Stats target | Description |
|------------------|-----------------------------|-----------|----------|------------------------------------------------------|----------|-------------|--------------|-------------|
| transaction_id   | integer                     |           | not null | nextval('transactions_transaction_id_seq'::regclass) | plain    |      |              | |
| transaction_hash | text                        |           | not null |                                                      | extended |     |              | |
| block_id         | integer                     |           |          |                                                      | plain    |     |              | |
| sender           | text                        |           |          |                                                      | extended |     |              | |
| receiver         | text                        |           |          |                                                      | extended |     |              | |
| amount           | numeric(20,8)               |           |          |                                                      | main     |     |              | |
| signature        | text                        |           |          |                                                      | extended |     |              | |
| created_at       | timestamp without time zone |           |          | CURRENT_TIMESTAMP                                    | plain    |     |              | |
| user_id          | integer                     |           |          |                                                      | plain    |     |              | |
| sender_id        | integer                     |           |          |                                                      | plain    |     |              | |
| receiver_id      | integer                     |           |          |                                                      | plain    |     |              | |

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


kvp_blockchain=# \d+ admin;

                                                                        Table "public.admin"
                                                                        
|    Column     |            Type             | Collation | Nullable |                 Default                 | Storage  | Compression | Stats target | Description |
|---------------|-----------------------------|-----------|----------|-----------------------------------------|----------|-------------|--------------|-------------|
| admin_id      | integer                     |           | not null | nextval('admin_admin_id_seq'::regclass) | plain    |             |    | |
| username      | character varying(50)       |           | not null |                                         | extended |             |    | |
| password_hash | text                        |           | not null |                                         | extended |             |    | |
| email         | character varying(100)      |           | not null |                                         | extended |             |    | |
| role          | character varying(20)       |           |          | 'admin'::character varying              | extended |             |    | |
| created_at    | timestamp without time zone |           |          | CURRENT_TIMESTAMP                       | plain    |             |    | |

Indexes:

    "admin_pkey" PRIMARY KEY, btree (admin_id)
    "admin_email_key" UNIQUE CONSTRAINT, btree (email)
    "admin_username_key" UNIQUE CONSTRAINT, btree (username)
    
Referenced by:

    TABLE "kyc_submissions" CONSTRAINT "fk_reviewed_by_admin" FOREIGN KEY (reviewed_by) REFERENCES admin(admin_id) ON DELETE SET NULL
    TABLE "kyc_approvals" CONSTRAINT "kyc_approvals_admin_id_fkey" FOREIGN KEY (admin_id) REFERENCES admin(admin_id)
    
Access method: heap


kvp_blockchain=# \d+ blocks;

                                                                          Table "public.blocks"
                                                                          
|      Column       |            Type             | Collation | Nullable |                 Default                  | Storage  | Compression | Stats target | Description |
|-------------------|-----------------------------|-----------|----------|------------------------------------------|----------|-------------|--------------|-------------|
| block_id          | integer                     |           | not null | nextval('blocks_block_id_seq'::regclass) | plain    |             |         | |
| block_hash        | text                        |           | not null |                                          | extended |             |         | |
| previous_hash     | text                        |           |          |                                          | extended |             |         | |
| timestamp         | timestamp without time zone |           |          | CURRENT_TIMESTAMP                        | plain    |             |         | |
| data              | jsonb                       |           |          |                                          | extended |             |         | |
| nonce             | integer                     |           |          |                                          | plain    |             |         | |
| created_at        | timestamp without time zone |           |          | CURRENT_TIMESTAMP                        | plain    |             |         | |
| transactions      | jsonb                       |           |          |                                          | extended |             |         | |
| total_blocks      | integer                     |           | not null | 0                                        | plain    |             |         | |
| active_blocks     | integer                     |           | not null | 0                                        | plain    |             |         | |
| inactive_blocks   | integer                     |           | not null | 0                                        | plain    |             |         | |
| burned_blocks     | integer                     |           |          | 0                                        | plain    |             |         | |
| remaining_blocks  | integer                     |           | not null | 0                                        | plain    |             |         | |
| halving_schedule  | timestamp without time zone |           |          |                                          | plain    |             |         | |
| last_block_time   | timestamp without time zone |           |          |                                          | plain    |             |         | |
| next_halving_time | timestamp without time zone |           |          |                                          | plain    |             |         | |
| last_burn_time    | timestamp without time zone |           |          |                                          | plain    |             |         | |
| burned            | boolean                     |           |          | false                                    | plain    |             |         | |
| burned_block_ids  | text[]                      |           |          |                                          | extended |             |         | |

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


kvp_blockchain=# \d+ cloud;

                                                                  Table "public.cloud"
                                                                  
|   Column   |           Type           | Collation | Nullable |              Default              | Storage  | Compression | Stats target | Description |
|------------|--------------------------|-----------|----------|-----------------------------------|----------|-------------|--------------|-------------|
| id         | integer                  |           | not null | nextval('cloud_id_seq'::regclass) | plain    |             |              | |
| file_path  | character varying(255)   |           | not null |                                   | extended |             |              | |
| size       | bigint                   |           | not null |                                   | plain    |             |              | |
| status     | character varying(50)    |           |          | 'active'::character varying       | extended |             |              | |
| created_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                 | plain    |             |              | |
| updated_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                 | plain    |             |              | |

Indexes:

    "cloud_pkey" PRIMARY KEY, btree (id)
    
Referenced by:

    TABLE "mirror" CONSTRAINT "mirror_cloud_id_fkey" FOREIGN KEY (cloud_id) REFERENCES cloud(id) ON DELETE CASCADE
    TABLE "peer" CONSTRAINT "peer_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    TABLE "seeder" CONSTRAINT "seeder_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    
Access method: heap


kvp_blockchain=# \d+ mirror;

                                                                  Table "public.mirror"
                                                                  
|   Column    |           Type           | Collation | Nullable |              Default               | Storage  | Compression | Stats target | Description |
|-------------|--------------------------|-----------|----------|------------------------------------|----------|-------------|--------------|-------------|
| id          | integer                  |           | not null | nextval('mirror_id_seq'::regclass) | plain    |             |              | |
| cloud_id    | integer                  |           |          |                                    | plain    |             |              | |
| mirror_path | character varying(255)   |           | not null |                                    | extended |             |              | |
| created_at  | timestamp with time zone |           |          | CURRENT_TIMESTAMP                  | plain    |             |              | |

Indexes:

    "mirror_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "mirror_cloud_id_fkey" FOREIGN KEY (cloud_id) REFERENCES cloud(id) ON DELETE CASCADE
    
Access method: heap


kvp_blockchain=# \d+ seeder;

                                                                  Table "public.seeder"
                                                                  
|   Column   |           Type           | Collation | Nullable |              Default               | Storage  | Compression | Stats target | Description |
|------------|--------------------------|-----------|----------|------------------------------------|----------|-------------|--------------|-------------|
| id         | integer                  |           | not null | nextval('seeder_id_seq'::regclass) | plain    |             |              | |
| user_id    | integer                  |           |          |                                    | plain    |             |              | |
| file_id    | integer                  |           |          |                                    | plain    |             |              | |
| status     | character varying(50)    |           |          | 'active'::character varying        | extended |             |              | |
| created_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                  | plain    |             |              | |

Indexes:

    "seeder_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "seeder_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    "seeder_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Access method: heap


kvp_blockchain=# \d+ peer;

                                                                  Table "public.peer"
                                                                  
|   Column   |           Type           | Collation | Nullable |             Default              | Storage  | Compression | Stats target | Description |
|------------|--------------------------|-----------|----------|----------------------------------|----------|-------------|--------------|-------------|
| id         | integer                  |           | not null | nextval('peer_id_seq'::regclass) | plain    |             |              | |
| user_id    | integer                  |           |          |                                  | plain    |             |              | |
| file_id    | integer                  |           |          |                                  | plain    |             |              | |
| status     | character varying(50)    |           |          | 'active'::character varying      | extended |             |              | |
| created_at | timestamp with time zone |           |          | CURRENT_TIMESTAMP                | plain    |             |              | |

Indexes:

    "peer_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "peer_file_id_fkey" FOREIGN KEY (file_id) REFERENCES cloud(id)
    "peer_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Triggers:

    peer_status_update AFTER UPDATE ON peer FOR EACH ROW EXECUTE FUNCTION update_peer_status()
    
Access method: heap


kvp_blockchain=# \d+ channel;

                                                                   Table "public.channel"
                                                                   
|    Column    |           Type           | Collation | Nullable |               Default               | Storage  | Compression | Stats target | Description |
|--------------|--------------------------|-----------|----------|-------------------------------------|----------|-------------|--------------|-------------|
| id           | integer                  |           | not null | nextval('channel_id_seq'::regclass) | plain    |             |              | |
| user_id      | integer                  |           |          |                                     | plain    |             |              | |
| access_point | character varying(255)   |           | not null |                                     | extended |             |              | |
| created_at   | timestamp with time zone |           |          | CURRENT_TIMESTAMP                   | plain    |             |              | |

Indexes:

    "channel_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "channel_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Access method: heap


kvp_blockchain=# \d+ dns;

                                                                  Table "public.dns"
                                                                  
|   Column    |           Type           | Collation | Nullable |             Default             | Storage  | Compression | Stats target | Description |
|-------------|--------------------------|-----------|----------|---------------------------------|----------|-------------|--------------|-------------|
| id          | integer                  |           | not null | nextval('dns_id_seq'::regclass) | plain    |             |              | |
| domain_name | character varying(255)   |           | not null |                                 | extended |             |              | |
| ip_address  | character varying(45)    |           |          |                                 | extended |             |              | |
| created_at  | timestamp with time zone |           |          | CURRENT_TIMESTAMP               | plain    |             |              | |

Indexes:

    "dns_pkey" PRIMARY KEY, btree (id)
    
Access method: heap


kvp_blockchain=# \d+ bandwidth;

                                                                    Table "public.bandwidth"
                                                                    
|     Column      |           Type           | Collation | Nullable |                Default                | Storage | Compression | Stats target | Description |
|-----------------|--------------------------|-----------|----------|---------------------------------------|---------|-------------|--------------|-------------|
| id              | integer                  |           | not null | nextval('bandwidth_id_seq'::regclass) | plain   |             | | |
| user_id         | integer                  |           |          |                                       | plain   |             | | |
| bandwidth_limit | integer                  |           | not null |                                       | plain   |             | | |
| usage           | integer                  |           | not null |                                       | plain   |             | | |
| created_at      | timestamp with time zone |           |          | CURRENT_TIMESTAMP                     | plain   |             | | |

Indexes:

    "bandwidth_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "bandwidth_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Triggers:

    update_bandwidth_usage_trigger AFTER INSERT ON bandwidth FOR EACH ROW EXECUTE FUNCTION update_bandwidth_usage()
    
Access method: heap


kvp_blockchain=# \d+ gateway;

                                                                  Table "public.gateway"
                                                                  
|   Column    |           Type           | Collation | Nullable |               Default               | Storage  | Compression | Stats target | Description |
|-------------|--------------------------|-----------|----------|-------------------------------------|----------|-------------|--------------|-------------|
| id          | integer                  |           | not null | nextval('gateway_id_seq'::regclass) | plain    |             |              | |
| user_id     | integer                  |           |          |                                     | plain    |             |              | |
| share_point | character varying(255)   |           | not null |                                     | extended |             |              | |
| created_at  | timestamp with time zone |           |          | CURRENT_TIMESTAMP                   | plain    |             |              | |

Indexes:

    "gateway_pkey" PRIMARY KEY, btree (id)
    
Foreign-key constraints:

    "gateway_user_id_fkey" FOREIGN KEY (user_id) REFERENCES users(user_id)
    
Triggers:

    update_gateway_share_trigger AFTER INSERT ON gateway FOR EACH ROW EXECUTE FUNCTION update_gateway_share()
    
Access method: heap


kvp_blockchain=# \d+ __diesel_schema_migrations;

                                               Table "public.__diesel_schema_migrations"
                                               
| Column  |            Type             | Collation | Nullable |      Default      | Storage  | Compression | Stats target | Description |
|---------|-----------------------------|-----------|----------|-------------------|----------|-------------|--------------|-------------|
| version | character varying(50)       |           | not null |                   | extended |             |              | |
| run_on  | timestamp without time zone |           | not null | CURRENT_TIMESTAMP | plain    |             |              | |

Indexes:

    "__diesel_schema_migrations_pkey" PRIMARY KEY, btree (version)
    
Access method: heap

kvp_blockchain=# \df

                                                      List of functions
                                                      
| Schema |           Name           | Result data type |                      Argument data types                      | Type |
|--------|--------------------------|------------------|---------------------------------------------------------------|------|
| public | add_peer                 | void             | user_id integer, file_id integer                              | func |
| public | add_reward_to_pool       | void             | p_user_id integer, p_pool_id integer, p_reward_amount numeric | func |
| public | admin_approve_kyc        | void             | user_id integer, approval_status character varying            | func |
| public | deduct_reward_from_pool  | void             | p_user_id integer, p_pool_id integer, p_reward_amount numeric | func |
| public | diesel_manage_updated_at | void             | _tbl regclass                                                 | func |
| public | diesel_set_updated_at    | trigger          |                                                               | func |
| public | submit_kyc               | void             | user_id integer                                               | func |
| public | update_bandwidth_usage   | trigger          |                                                               | func |
| public | update_block_stats       | trigger          |                                                               | func |
| public | update_gateway_share     | trigger          |                                                               | func |
| public | update_peer_status       | trigger          |                                                               | func |
| public | verify_kyc_automatically | trigger          |                                                               | func |
| public | verify_kyc_automatically | void             | user_id integer                                               | func |

(13 rows)


kvp_blockchain=# \dy

              List of event triggers
              
| Name | Event | Owner | Enabled | Function | Tags |
|------|-------|-------|---------|----------|------|
| | | | | | |

(0 rows)

kvp_blockchain=# \di

                                     List of relations
                                     
| Schema |               Name                | Type  |  Owner   |           Table            |
|--------|-----------------------------------|-------|----------|----------------------------|
| public | __diesel_schema_migrations_pkey   | index | postgres | __diesel_schema_migrations |
| public | admin_email_key                   | index | postgres | admin |
| public | admin_pkey                        | index | postgres | admin |
| public | admin_username_key                | index | postgres | admin |
| public | bandwidth_pkey                    | index | postgres | bandwidth |
| public | blocks_block_hash_key             | index | postgres | blocks |
| public | blocks_pkey                       | index | postgres | blocks |
| public | channel_pkey                      | index | postgres | channel |
| public | cloud_pkey                        | index | postgres | cloud |
| public | dns_pkey                          | index | postgres | dns |
| public | game_rewards_pkey                 | index | postgres | game_rewards |
| public | game_sessions_pkey                | index | postgres | game_sessions |
| public | games_pkey                        | index | postgres | games |
| public | gateway_pkey                      | index | postgres | gateway |
| public | idx_block_id                      | index | postgres | transactions |
| public | idx_user_id                       | index | postgres | transactions |
| public | kyc_approvals_pkey                | index | postgres | kyc_approvals |
| public | kyc_submissions_pkey              | index | postgres | kyc_submissions |
| public | marketplace_pkey                  | index | postgres | marketplace |
| public | mining_rewards_pkey               | index | postgres | mining_rewards |
| public | mirror_pkey                       | index | postgres | mirror |
| public | nfts_pkey                         | index | postgres | nfts |
| public | peer_pkey                         | index | postgres | peer |
| public | pool_allocations_pkey             | index | postgres | pool_allocations |
| public | proposals_pkey                    | index | postgres | proposals |
| public | reward_pools_pkey                 | index | postgres | reward_pools |
| public | seeder_pkey                       | index | postgres | seeder |
| public | sessions_pkey                     | index | postgres | sessions |
| public | tracker_pkey                      | index | postgres | tracker |
| public | transactions_pkey                 | index | postgres | transactions |
| public | transactions_transaction_hash_key | index | postgres | transactions |
| public | user_registrations_pkey           | index | postgres | user_registrations |
| public | users_email_key                   | index | postgres | users |
| public | users_pkey                        | index | postgres | users |
| public | users_username_key                | index | postgres | users |
| public | votes_pkey                        | index | postgres | votes |

(36 rows)

kvp_blockchain=# \di

                                     List of relations
                                     
| Schema |               Name                | Type  |  Owner   |           Table            |
|--------|-----------------------------------|-------|----------|----------------------------|
| public | __diesel_schema_migrations_pkey   | index | postgres | __diesel_schema_migrations |
| public | admin_email_key                   | index | postgres | admin |
| public | admin_pkey                        | index | postgres | admin |
| public | admin_username_key                | index | postgres | admin |
| public | bandwidth_pkey                    | index | postgres | bandwidth |
| public | blocks_block_hash_key             | index | postgres | blocks |
| public | blocks_pkey                       | index | postgres | blocks |
| public | channel_pkey                      | index | postgres | channel |
| public | cloud_pkey                        | index | postgres | cloud |
| public | dns_pkey                          | index | postgres | dns |
| public | game_rewards_pkey                 | index | postgres | game_rewards |
| public | game_sessions_pkey                | index | postgres | game_sessions |
| public | games_pkey                        | index | postgres | games |
| public | gateway_pkey                      | index | postgres | gateway |
| public | idx_block_id                      | index | postgres | transactions |
| public | idx_user_id                       | index | postgres | transactions |
| public | kyc_approvals_pkey                | index | postgres | kyc_approvals |
| public | kyc_submissions_pkey              | index | postgres | kyc_submissions |
| public | marketplace_pkey                  | index | postgres | marketplace |
| public | mining_rewards_pkey               | index | postgres | mining_rewards |
| public | mirror_pkey                       | index | postgres | mirror |
| public | nfts_pkey                         | index | postgres | nfts |
| public | peer_pkey                         | index | postgres | peer |
| public | pool_allocations_pkey             | index | postgres | pool_allocations |
| public | proposals_pkey                    | index | postgres | proposals |
| public | reward_pools_pkey                 | index | postgres | reward_pools |
| public | seeder_pkey                       | index | postgres | seeder |
| public | sessions_pkey                     | index | postgres | sessions |
| public | tracker_pkey                      | index | postgres | tracker |
| public | transactions_pkey                 | index | postgres | transactions |
| public | transactions_transaction_hash_key | index | postgres | transactions |
| public | user_registrations_pkey           | index | postgres | user_registrations |
| public | users_email_key                   | index | postgres | users |
| public | users_pkey                        | index | postgres | users |
| public | users_username_key                | index | postgres | users |
| public | votes_pkey                        | index | postgres | votes |

(36 rows)


kvp_blockchain=# \dn

      List of schemas
      
|  Name  |       Owner       |
|--------|-------------------|
| public | pg_database_owner |

(1 row)

---

## **Penyesuaian Struktur Tabel Menurut PostgreSQL 28 Nov 2024**

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

├── dao/ 

│ ├── mod.rs 

│ ├── vote.rs 

│ └── proposal.rs 

├── utility/ 

│ ├──defi.rs 

│ ├── gamefi.rs

│ ├── nft.rs 

│ ├── marketplace.rs 

│ ├── storage/

│ │  ├── cloud.rs

│ │  ├── mirror.rs

│ │  ├── peer.rs

│ │  ├── seeder.rs

│ │  └── tracker.rs

│ ├── kvpnet/

│ │  ├── bandwith.rs

│ │  ├── channel.rs

│ │  ├── dns.rs

│ │  └── gateway.rs

├── mining/ 

│ └── mining.rs

├── pool/ 

│ └── pool.rs 

├── middleware/ 

│ └── middleware.rs // Opsional

├── db/ 

│ ├── mod.rs 

│ └── connection.rs 

└── main.rs
