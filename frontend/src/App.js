import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css'; // Make sure to include any necessary CSS for styling

function App() {
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  useEffect(() => {
    axios.get('http://localhost:8080/data')
      .then(response => {
        setUsers(response.data);
        setLoading(false);
      })
      .catch(err => {
        setError(err.message);
        setLoading(false);
      });
  }, []);

  if (error) {
    return <div className="error">Error: {error}</div>;
  }

  return (
    <div className="App">
      <h1>Data from API</h1>
      {loading ? (
        <div className="loading">Loading...</div>
      ) : (
        <div className="card-container">
          {users.map(user => (
            <div className="card" key={user.id}>
              <ul>
                <li><strong>Name:</strong> {user.name}</li>
                <li><strong>Age:</strong> {user.age}</li>
                <li><strong>Email:</strong> {user.email}</li>
              </ul>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default App;
