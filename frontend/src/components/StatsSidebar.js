/*
 * KVP Blockchain - Created from scratch by Kraken (Hendri RH) and Bro CG
 * 
 * This project is a groundbreaking blockchain solution focused on:
 * - Lightweight and efficient design
 * - High-speed transactions
 * - Environmentally friendly and resource-optimized mechanisms
 * 
 * Built with React.js to ensure scalability and user-friendliness.
 * 
 * All rights reserved © 2024 Kraken & Bro CG
 */

import React, { useState, useEffect } from "react";

const StatsSidebar = () => {
  const [stats, setStats] = useState({
    total_blocks: 0,
    total_transactions: 0,
    reward_pool: 0,
  });

  useEffect(() => {
    fetch("http://127.0.0.1:3030/stats")
      .then((response) => response.json())
      .then((data) => setStats(data))
      .catch((error) => console.error("Error fetching stats:", error));
  }, []);

  const sidebarStyle = {
    position: "fixed", // Tetap di posisi tetap
    right: "20px", // Geser ke kiri dengan jarak 20px
    top: "80px", // Jarak dari atas
    width: "250px",
    backgroundColor: "#f9f9f9",
    border: "1px solid #ddd",
    borderRadius: "10px",
    padding: "15px",
    boxShadow: "0 4px 6px rgba(0, 0, 0, 0.1)",
    fontSize: "16px",
  };

  const titleStyle = {
    fontWeight: "bold",
    fontSize: "18px",
    marginBottom: "10px",
  };

  return (
    <div style={sidebarStyle}>
      <div style={titleStyle}>Statistics</div>
      <p><strong>Total Blocks:</strong> {stats.total_blocks}</p>
      <p><strong>Total Transactions:</strong> {stats.total_transactions}</p>
      <p><strong>Reward Pool:</strong> {stats.reward_pool}</p>
    </div>
  );
};

export default StatsSidebar;
