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
import { useNavigate } from "react-router-dom";

const LoginForm = () => {
  const [usernameOrEmail, setUsernameOrEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const navigate = useNavigate();

  const handleSubmit = async (e) => {
    e.preventDefault();
    setError(""); // Reset pesan error

    try {
      const response = await fetch("http://127.0.0.1:3030/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username_or_email: usernameOrEmail, password }),
      });

      if (!response.ok) {
        if (response.status === 401) {
          throw new Error("Invalid login credentials. Please check your username and password.");
        }
        if (response.status === 404) {
          throw new Error("API endpoint not found. Please check the backend route.");
        }
        throw new Error("Failed to login");
      }

      const data = await response.json();
      localStorage.setItem("authToken", data.token); // Simpan token ke localStorage
      navigate("/"); // Redirect ke halaman utama
    } catch (err) {
      setError(err.message);
    }
  };

  return (
    <div style={{ maxWidth: "400px", margin: "0 auto", textAlign: "center" }}>
      <h2>Login</h2>
      {error && <p style={{ color: "red" }}>{error}</p>}
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          placeholder="Username or Email"
          value={usernameOrEmail}
          onChange={(e) => setUsernameOrEmail(e.target.value)}
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
        <button type="submit" style={{ marginTop: "10px" }}>Login</button>
      </form>
    </div>
  );
};

export default LoginForm;
