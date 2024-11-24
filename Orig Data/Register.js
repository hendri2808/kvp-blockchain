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

import React, { useState } from "react";

const RegisterForm = () => {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");

  const handleSubmit = async (e) => {
    e.preventDefault();
    setMessage(""); // Reset pesan error

    try {
      const response = await fetch("http://127.0.0.1:3030/register", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username, email, password }),
      });

      if (!response.ok) {
        if (response.status === 404) {
          throw new Error("API endpoint not found. Please check the backend route.");
        }
        throw new Error("Failed to register user");
      }

      const data = await response.json();
      setMessage(`Registration successful! User ID: ${data.user_id}`);
    } catch (err) {
      setMessage(`Error: ${err.message}`);
    }
  };

  return (
    <div style={{ maxWidth: "400px", margin: "0 auto", textAlign: "center" }}>
      <h2>Register</h2>
      {message && <p>{message}</p>}
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          required
          style={{ display: "block", margin: "10px auto", width: "100%" }}
        />
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
          style={{ display: "block", margin: "10px auto", width: "100%" }}
        />
        <input
          type="password"
          placeholder="Password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
          style={{ display: "block", margin: "10px auto", width: "100%" }}
        />
        <button type="submit" style={{ marginTop: "10px" }}>Register</button>
      </form>
    </div>
  );
};

export default RegisterForm;
