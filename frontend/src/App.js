import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css'; // Make sure to include any necessary CSS for styling

function App() {
  // State to store the fetched data
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  // Fetch data from API when component mounts
  useEffect(() => {
    axios.get('http://localhost:8080/data')
      .then(response => {
        setData(response.data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, []);

  if (loading) {
    return <div className="loading">Loading...</div>;
  }

  if (error) {
    return <div className="error">Error: {error}</div>;
  }

  return (
    <div className="App">
      <h1>Data from API</h1>
      {data && (
        <ul>
          <li><strong>Name:</strong> {data.name}</li>
          <li><strong>Age:</strong> {data.age}</li>
          <li><strong>Email:</strong> {data.email}</li>
        </ul>
      )}
    </div>
  );
}

export default App;
