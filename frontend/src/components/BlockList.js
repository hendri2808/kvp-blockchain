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
import { useNavigate } from "react-router-dom";

const BlockList = () => {
  const [blocks, setBlocks] = useState([]);
  const [currentPage, setCurrentPage] = useState(1);
  const [itemsPerPage, setItemsPerPage] = useState(10);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  const [searchTerm, setSearchTerm] = useState("");
  const [searchResults, setSearchResults] = useState([]);
  const [sortBy, setSortBy] = useState("index");
  const [sortOrder, setSortOrder] = useState("asc");
  const [filterByTransactions, setFilterByTransactions] = useState("");

  const navigate = useNavigate();

  useEffect(() => {
    setLoading(true);
  
    // Fetch data awal dari REST API
    fetch("http://127.0.0.1:3030/blockchain")
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch blockchain data.");
        }
        return response.json();
      })
      .then((data) => setBlocks(data))
      .catch((error) => setError(error.message))
      .finally(() => setLoading(false));
  
    // Inisialisasi WebSocket untuk realtime update
    const ws = new WebSocket("ws://127.0.0.1:3030/ws");
  
    ws.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data);
        console.log("Received JSON data:", data);
        // Lakukan sesuatu dengan data JSON
      } catch (error) {
        console.warn("Received non-JSON message:", event.data);
        // Jika bukan JSON, abaikan atau lakukan sesuatu dengan pesan tersebut
      }
    };    
  
    return () => ws.close(); // Bersihkan koneksi WebSocket saat komponen di-unmount
  }, []);

  const handleSearch = (e) => {
    setSearchTerm(e.target.value);
    if (e.target.value) {
      fetch(`http://127.0.0.1:3030/search?q=${e.target.value}`)
        .then((response) => {
          if (!response.ok) {
            throw new Error("Search failed.");
          }
          return response.json();
        })
        .then((data) => setSearchResults(data))
        .catch((error) => console.error(error));
    } else {
      setSearchResults([]);
    }
  };

  const handleSelectResult = (index) => {
    navigate(`/block/${index}`);
    setSearchTerm("");
    setSearchResults([]);
  };

  const filteredAndSortedBlocks = blocks
    .filter((block) => {
      if (filterByTransactions) {
        return block.transactions.length >= filterByTransactions;
      }
      return true;
    })
    .sort((a, b) => {
      if (sortBy === "index") {
        return sortOrder === "asc" ? a.index - b.index : b.index - a.index;
      } else if (sortBy === "timestamp") {
        return sortOrder === "asc"
          ? new Date(a.timestamp) - new Date(b.timestamp)
          : new Date(b.timestamp) - new Date(a.timestamp);
      }
      return 0;
    });

  const totalPages = Math.ceil(filteredAndSortedBlocks.length / itemsPerPage);
  const startIndex = (currentPage - 1) * itemsPerPage;
  const paginatedBlocks = filteredAndSortedBlocks.slice(startIndex, startIndex + itemsPerPage);

  const handleItemsPerPageChange = (e) => {
    setItemsPerPage(Number(e.target.value));
    setCurrentPage(1);
  };

  const handleSortChange = (e) => {
    setSortBy(e.target.value);
  };

  const handleSortOrderChange = (e) => {
    setSortOrder(e.target.value);
  };

  const handleFilterByTransactionsChange = (e) => {
    setFilterByTransactions(Number(e.target.value));
  };

  if (loading) {
    return <div style={{ textAlign: "center", marginTop: "20px" }}>Loading...</div>;
  }

  if (error) {
    return <div style={{ textAlign: "center", marginTop: "20px", color: "red" }}>{error}</div>;
  }

  return (
    <div style={{ margin: "20px" }}>
      <h2 style={{ textAlign: "center", marginBottom: "20px" }}>Blockchain Blocks</h2>

      {/* Search Bar */}
      <div style={{ textAlign: "center", marginBottom: "20px" }}>
        <input
          type="text"
          placeholder="Search by hash or index..."
          value={searchTerm}
          onChange={handleSearch}
          style={{
            width: "100%",
            maxWidth: "400px",
            padding: "10px",
            border: "1px solid #ccc",
            borderRadius: "5px",
          }}
        />
        {searchResults.length > 0 && (
          <div
            style={{
              position: "absolute",
              backgroundColor: "#fff",
              boxShadow: "0 2px 5px rgba(0,0,0,0.2)",
              width: "100%",
              maxWidth: "400px",
              margin: "0 auto",
              zIndex: 1000,
            }}
          >
            {searchResults.map((block) => (
              <div
                key={block.index}
                style={{
                  padding: "10px",
                  borderBottom: "1px solid #ccc",
                  cursor: "pointer",
                }}
                onClick={() => handleSelectResult(block.index)}
              >
                {block.index} - {block.hash.substring(0, 10)}...
              </div>
            ))}
          </div>
        )}
      </div>

      {/* Sorting and Filtering */}
      <div style={{ display: "flex", justifyContent: "space-between", marginBottom: "20px" }}>
        <div>
          <label>Sort by: </label>
          <select value={sortBy} onChange={handleSortChange}>
            <option value="index">Index</option>
            <option value="timestamp">Timestamp</option>
          </select>
          <select value={sortOrder} onChange={handleSortOrderChange}>
            <option value="asc">Ascending</option>
            <option value="desc">Descending</option>
          </select>
        </div>
        <div>
          <label>Min Transactions: </label>
          <input
            type="number"
            value={filterByTransactions}
            onChange={handleFilterByTransactionsChange}
            placeholder="0"
          />
        </div>
      </div>

      {/* Pagination */}
      <div style={{ marginBottom: "10px", textAlign: "center" }}>
        <label htmlFor="itemsPerPage">Items per page: </label>
        <select
          id="itemsPerPage"
          value={itemsPerPage}
          onChange={handleItemsPerPageChange}
          style={{ padding: "5px", marginLeft: "5px" }}
        >
          <option value={10}>10</option>
          <option value={50}>50</option>
          <option value={100}>100</option>
        </select>
      </div>

      {/* Blocks Table */}
      <table style={{ width: "100%", borderCollapse: "collapse", margin: "0 auto" }}>
        <thead>
          <tr style={{ backgroundColor: "#4a90e2", color: "#ffffff" }}>
            <th style={{ padding: "10px", border: "1px solid #ddd" }}>Index</th>
            <th style={{ padding: "10px", border: "1px solid #ddd" }}>Hash</th>
            <th style={{ padding: "10px", border: "1px solid #ddd" }}>Timestamp</th>
            <th style={{ padding: "10px", border: "1px solid #ddd" }}>Transactions</th>
          </tr>
        </thead>
        <tbody>
          {paginatedBlocks.map((block) => (
            <tr
              key={block.index}
              style={{
                textAlign: "center",
                backgroundColor: block.index % 2 === 0 ? "#f9f9f9" : "#ffffff",
                cursor: "pointer",
              }}
              onClick={() => navigate(`/block/${block.index}`)}
            >
              <td style={{ padding: "10px", border: "1px solid #ddd" }}>{block.index}</td>
              <td style={{ padding: "10px", border: "1px solid #ddd", textAlign: "left" }}>
                {block.hash}
              </td>
              <td style={{ padding: "10px", border: "1px solid #ddd" }}>
                {new Date(block.timestamp).toLocaleString()}
              </td>
              <td style={{ padding: "10px", border: "1px solid #ddd" }}>
                {block.transactions.length}
              </td>
            </tr>
          ))}
        </tbody>
      </table>

      {/* Pagination Buttons */}
      <div style={{ marginTop: "20px", textAlign: "center" }}>
        <button
          onClick={() => setCurrentPage((prev) => Math.max(prev - 1, 1))}
          disabled={currentPage === 1}
          style={{
            padding: "10px 20px",
            marginRight: "10px",
            backgroundColor: currentPage === 1 ? "#cccccc" : "#4a90e2",
            color: "#ffffff",
            border: "none",
            cursor: currentPage === 1 ? "not-allowed" : "pointer",
          }}
        >
          Previous
        </button>
        <button
          onClick={() => setCurrentPage((prev) => Math.min(prev + 1, totalPages))}
          disabled={currentPage === totalPages}
          style={{
            padding: "10px 20px",
            backgroundColor: currentPage === totalPages ? "#cccccc" : "#4a90e2",
            color: "#ffffff",
            border: "none",
            cursor: currentPage === totalPages ? "not-allowed" : "pointer",
          }}
        >
          Next
        </button>
      </div>
    </div>
  );
};

export default BlockList;
