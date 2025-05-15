import React, { useEffect, useState } from "react";
import { fetchAllItems } from "./utils/fetchItems"; // adjust the path to your fetch function
import BuildUI from "./components/BuildUI";

function App() {
  const [items, setItems] = useState(null);
  const [error, setError] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function loadItems() {
      try {
        const data = await fetchAllItems();
        setItems(data);
      } catch (err) {
        console.error("Error fetching items:", err);
        setError("Could not load items. Please refresh the page.");
      } finally {
        setLoading(false);
      }
    }
    loadItems();
  }, []);

  if (loading) {
    return (
      <div style={{ padding: "2rem", textAlign: "center" }}>
        <h1>Loading items...</h1>
      </div>
    );
  }

  if (error) {
    return (
      <div style={{ padding: "2rem", textAlign: "center", color: "red" }}>
        <h1>{error}</h1>
      </div>
    );
  }

  return (
    <div style={{ maxWidth: "960px", margin: "0 auto", padding: "1rem" }}>
      <header style={{ marginBottom: "2rem" }}>
        <h1>Armor and Weapons Builder</h1>
        <p>Select your gear from the available items below.</p>
      </header>

      <BuildUI
        helmets={items.helmets || []}
        chestplates={items.chestplates || []}
        leggings={items.leggings || []}
        boots={items.boots || []}
        rings={items.rings || []}
        bracelets={items.bracelets || []}
        necklaces={items.necklaces || []}
        weapons={items.weapons || []}
      />

      <footer style={{ marginTop: "3rem", textAlign: "center", fontSize: "0.9rem", color: "#555" }}>
        <p>© 2025 Your Game Company</p>
      </footer>
    </div>
  );
}

export default App;

