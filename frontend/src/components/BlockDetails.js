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

import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";

const BlockDetails = () => {
  const { index } = useParams();
  const navigate = useNavigate();
  const [block, setBlock] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    setLoading(true);
    fetch(`http://127.0.0.1:3030/block/${index}`)
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch block details.");
        }
        return response.json();
      })
      .then((data) => setBlock(data))
      .catch((error) => setError(error.message))
      .finally(() => setLoading(false));
  }, [index]);

  if (loading) {
    return <div style={{ textAlign: "center", marginTop: "20px" }}>Loading...</div>;
  }

  if (error) {
    return <div style={{ textAlign: "center", marginTop: "20px", color: "red" }}>{error}</div>;
  }

  if (!block) {
    return <div style={{ textAlign: "center", marginTop: "20px" }}>Block not found.</div>;
  }

  // Styling
  const containerStyle = {
    textAlign: "center",
    margin: "20px",
    padding: "10px",
  };

  const cardStyle = {
    maxWidth: "600px",
    margin: "0 auto",
    padding: "20px",
    border: "1px solid #ddd",
    borderRadius: "8px",
    backgroundColor: "#f9f9f9",
    wordBreak: "break-word",
  };

  const transactionStyle = {
    textAlign: "left",
    margin: "10px 0",
    wordBreak: "break-word",
  };

  const backButtonStyle = {
    display: "inline-block",
    marginBottom: "20px",
    padding: "10px 20px",
    backgroundColor: "#4a90e2",
    color: "#fff",
    textDecoration: "none",
    borderRadius: "5px",
    cursor: "pointer",
  };

  return (
    <div style={containerStyle}>
      {/* Back Button */}
      <button
        style={backButtonStyle}
        onClick={() => navigate("/")}
      >
        Back to Home
      </button>

      <h2>Block Details</h2>
      <div style={cardStyle}>
        <p><strong>Index:</strong> {block.index}</p>
        <p><strong>Hash:</strong> {block.hash}</p>
        <p><strong>Timestamp:</strong> {new Date(block.timestamp).toLocaleString()}</p>
        <p><strong>Previous Hash:</strong> {block.previous_hash}</p>
        <p><strong>Transactions:</strong></p>
        <ul style={{ listStyleType: "none", padding: 0 }}>
          {block.transactions.map((tx, i) => (
            <li key={i} style={transactionStyle}>
              <strong>Sender:</strong> {tx.sender}<br />
              <strong>Receiver:</strong> {tx.receiver}<br />
              <strong>Amount:</strong> {tx.amount}<br />
              <strong>Contribution:</strong> {tx.contribution}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default BlockDetails;
