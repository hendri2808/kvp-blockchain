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
import { useNavigate } from "react-router-dom"; // Tambahkan import

const LoginForm = () => {
  const [usernameOrEmail, setUsernameOrEmail] = useState("");
  const [password, setPassword] = useState("");
  const [message, setMessage] = useState("");
  const navigate = useNavigate(); // Pindahkan ke dalam fungsi komponen

  const handleLogin = async (e) => {
    e.preventDefault();
    setMessage("Logging in...");

    if (!usernameOrEmail || !password) {
      setMessage("Please fill in all fields");
      return;
    }

    try {
      const response = await fetch("http://127.0.0.1:3030/login", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ username_or_email: usernameOrEmail, password }),
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || "Login failed");
      }

      const data = await response.json();
      console.log("Response:", data); // Debugging untuk memeriksa respons

      setMessage("Login successful!");
      localStorage.setItem("authToken", data.token); // Simpan token untuk otentikasi
      navigate("/dashboard"); // Redirect ke dashboard
    } catch (err) {
      setMessage(`Error: ${err.message}`);
    }
  };

  return (
    <div style={{ maxWidth: "400px", margin: "0 auto", textAlign: "center" }}>
      <h2>Login</h2>
      {message && <p style={{ color: message.includes("Error") ? "red" : "green" }}>{message}</p>}
      <form onSubmit={handleLogin}>
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
