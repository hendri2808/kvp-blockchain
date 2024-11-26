# KVP Blockchain Project

### Created from scratch by **Kraken (Hendri RH)** and **Bro CG**
Progress November 24, 2024

---

## Project Overview

The KVP Blockchain is a groundbreaking blockchain solution focused on:
- **Lightweight and efficient design**: Designed to run efficiently on both high-performance and lightweight devices.
- **High-speed transactions**: Optimized for rapid transaction processing.
- **Environmentally friendly and resource-optimized mechanisms**: Aiming to minimize energy consumption and computational demands.

This blockchain is built with Rust to ensure **high performance and reliability**.

---

## Key Features and Progress

### KVP Protocol Design
- **Proof of Contribution**: Contribution-based validator mechanism focusing on real activities (e.g., transaction validation, uptime, etc.).
- **Mobile-Friendly Mining**: Lightweight mining suitable for mobile devices.
- **Decentralized Governance**: Utilizing DAO for community-based decision-making.
- **Web3, DeFi, NFT, and Game Integration**: Fully supports decentralized applications.

---

## Current Features (Completed ✅)
1. **Blockchain Explorer**:
   - Error handling and responsive design.
   - Highlight active block and real-time updates.
   - Sorting and filtering features.
   - Pagination and advanced search.
2. **User Authentication**:
   - Secure login and registration system.
3. **KYC Automation**:
   - Automated document validation using OCR.
   - Notification for admin in case of special manual review.
4. **Initial Backend APIs**:
   - Supporting basic blockchain and KYC functionalities.

---

## Pending Tasks
### Backend Development
- **Dashboard Admin**:
  - Approval KYC: Auto-handled except in special cases.
  - Reward Pool Management: Adjust rewards in case of crashes or disputes.
  - Reset User Passwords: Handle user requests securely.
  - Delete User: Burn all mined blocks and coins upon deletion.
  - Blockchain Statistics: Display user, miner, node, and block statistics.
- **Dashboard User**:
  - KYC Explorer: Dynamic form with real-time validation and document attachment.
  - Mining Interface: Display mined blocks and progress.
  - Wallet System: Handle rewards and transactions.

### Additional Features
- CI/CD Pipeline.
- Monitoring and logging using Grafana or Elastic Stack.
- NFT and Smart Contract Integration.
- API SDKs for multiple programming languages.

---

## Technical Roadmap

1. **Backend** (Priority):
   - API for admin and user dashboard functionalities.
   - Automation for KYC validations.
   - Real-time system statistics.

2. **Frontend**:
   - Intuitive interfaces for both admin and user dashboards.
   - Dynamic KYC forms and wallet systems.

3. **Integration Goals**:
   - DeFi integration for staking, lending, and liquidity pools.
   - Game support via NFTs and token economies.

---

## Progress Checklist 26 Nov 2024

| **No** | **Feature/Task**                                | **Description**                                                             | **Status**    					|
|--------|-------------------------------------------------|-----------------------------------------------------------------------------|----------------------------------|
| 1      | KVP Protocol Design                             | Design unique protocol with Proof of Contribution and governance mechanisms | ✅ Completed  					|
| 2      | Basic Blockchain and Explorer                   | Initial blockchain functionalities and explorer features                    | ✅ Completed  					|
| 3      | User Authentication                             | Secure login and registration                                               | ✅ Completed  					|
| 4      | KYC Automation                                  | Automated document validation using OCR                                     | [■■■■■■■□□□] 70% Loading...     	|
|        |                                                 | ....Doing Backend Work....                                                  | 🚀 Progress   					|
| 5      | Admin Dashboard                                 | Approval KYC, reward pool management, reset passwords, and statistics       | ⏳ Waiting     					|
| 6      | User Dashboard                                  | KYC Explorer, mining interface, wallet system                               | ⏳ Waiting      					|
| 7      | CI/CD Pipeline                                  | Automate deployment with GitHub Actions                                     | ⏳ Waiting      					|
| 8      | Monitoring and Logging                          | Real-time system monitoring and logging                                     | ⏳ Waiting     					|
| 9      | Advanced Filtering                              | Add advanced filters for explorer                                           | ⏳ Waiting      					|
| 10     | Dynamic KYC Explorer                            | Country-specific KYC forms with document validation                         | ⏳ Waiting     					|
| 11     | Wallet System                                   | Store and transact coins within the blockchain                              | ⏳ Waiting      					|
| 12     | DeFi and Game Integration                       | Staking, lending, NFTs, and token economy                                   | ⏳ Waiting     					|

---

## Deployment Instructions

1. **Backend**:
   - Clone the repository and navigate to the backend directory.
   - Run `cargo build` and `cargo run` to start the server.
2. **Frontend**:
   - Navigate to the frontend directory and install dependencies with `npm install`.
   - Start the frontend server using `npm start`.

---

## Contributing
We welcome contributions! To get started:
1. Fork the repository.
2. Create a new branch (`git checkout -b feature-name`).
3. Commit your changes and push the branch (`git push origin feature-name`).
4. Submit a pull request for review.

---

## License
All rights reserved © 2024 Kraken (Hendri RH) & Bro CG.

---

## Contact
For further inquiries, feel free to reach out via email:
- **Kraken (Hendri RH)**: rahmathst99@gmail.com
- **Bro CG**: Anonymous. Person who dont want anyome knows about him
- **Our Profile**: https://krakenteam.netlify.app/
