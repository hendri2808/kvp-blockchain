/*
 * KVP Blockchain - Created from scratch by Kraken (Hendri RH) and Bro CG
 * 
 * Built with React.js to ensure scalability and user-friendliness.
 * 
 * All rights reserved © 2024 Kraken & Bro CG
 */

import React, { useState } from "react";

const KycForm = () => {
  const [documentType, setDocumentType] = useState("Passport");
  const [file, setFile] = useState(null);
  const [message, setMessage] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleKycSubmit = async (e) => {
    e.preventDefault();

    if (!file) {
      setMessage("Please upload a file.");
      return;
    }

    // Validasi tipe file
    const allowedTypes = ["application/pdf", "image/jpeg", "image/png"];
    if (!allowedTypes.includes(file.type)) {
      setMessage("Invalid file type. Only PDF, JPEG, and PNG are allowed.");
      return;
    }

    const formData = new FormData();
    formData.append("docType", documentType);
    formData.append("document", file);

    try {
      setIsSubmitting(true); // Aktifkan loading
      const token = localStorage.getItem("authToken");
      const response = await fetch("http://127.0.0.1:3030/kyc", {
        method: "POST",
        headers: {
          Authorization: `Bearer ${token}`,
        },
        body: formData,
      });

      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.error || "KYC submission failed.");
      }

      const data = await response.json();
      setMessage(`Success: ${data.message}`);
    } catch (err) {
      setMessage(`Error: ${err.message}`);
    } finally {
      setIsSubmitting(false); // Matikan loading
    }
    console.log("Sending KYC FormData:", documentType, file);
  };

  return (
    <div style={{ textAlign: "center", marginTop: "20px" }}>
      <h2>KYC Form</h2>
      {message && <p style={{ color: message.includes("Error") ? "red" : "green" }}>{message}</p>}
      <form onSubmit={handleKycSubmit}>
        <select
          value={documentType}
          onChange={(e) => setDocumentType(e.target.value)}
          style={{ display: "block", margin: "10px auto", width: "100%" }}
        >
          <option value="Passport">Passport</option>
          <option value="National ID">National ID</option>
        </select>
        <input
          type="file"
          onChange={(e) => setFile(e.target.files[0])}
          style={{ display: "block", margin: "10px auto", width: "100%" }}
        />
        <button type="submit" disabled={isSubmitting} style={{ marginTop: "10px" }}>
          {isSubmitting ? "Submitting..." : "Submit KYC"}
        </button>
      </form>
    </div>
  );
};

export default KycForm;
