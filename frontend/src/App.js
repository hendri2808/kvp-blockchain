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

import React from "react";
import BlockList from "./components/BlockList";
import StatsSidebar from "./components/StatsSidebar";
import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import BlockDetails from "./components/BlockDetails";
import KYCForm from "./components/KYCForm";
import LoginForm from "./components/LoginForm";
import RegisterForm from "./components/RegisterForm";
import logo from "./logo.jpg";
import Dashboard from "./components/Dashboard"; // Buat file ini

function App() {
  return (
    <Router>
      <div style={{ display: "flex", height: "100vh" }}>
        {/* Sidebar untuk statistik */}
        <StatsSidebar />

        {/* Konten utama */}
        <div style={{ flex: 1, overflowY: "auto", padding: "20px", textAlign: "center" }}>
          {/* Header */}
          <div
            style={{
              display: "flex",
              flexDirection: "column", // Atur tombol dan logo secara vertikal
              alignItems: "center", // Posisikan semua elemen di tengah horizontal
              gap: "15px", // Beri jarak antar elemen
              marginBottom: "20px",
            }}
          >
            {/* Logo di Tengah */}
            <img
              src={logo}
              alt="KVP Logo"
              style={{
                width: "150px",
                height: "auto",
                borderRadius: "50%", // Membuat logo berbentuk lingkaran
              }}
            />

            {/* Tombol di Bawah Logo */}
            <div
              style={{
                display: "flex",
                gap: "10px", // Memberikan jarak antar tombol
                justifyContent: "center", // Posisikan tombol di tengah
              }}
            >
              <Link to="/register">
                <button>Register</button>
              </Link>
              <Link to="/login">
                <button>Login</button>
              </Link>
              <Link to="/kyc">
                <button>KYC</button>
              </Link>
            </div>
          </div>

          {/* Konten Utama */}
          <h1>KVP Blockchain Explorer</h1>
          <p>Welcome to the Kraken Velocity Protocol Blockchain Explorer!</p>

          <Routes>
            <Route path="/" element={<BlockList />} />
            <Route path="/block/:index" element={<BlockDetails />} />
            <Route path="/kyc" element={<KYCForm />} />
            <Route path="/login" element={<LoginForm />} />
            <Route path="/register" element={<RegisterForm />} />
            <Route path="/dashboard" element={<Dashboard />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;
