import React, { useState, useEffect } from 'react';
import axios from 'axios';
import './App.css';

function App() {
  const [users, setUsers] = useState([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);

  
  /* Mocking ("npm start" without backend)
  // Mock data
  const mockData = [
    { id: 1, name: "Alice", age: 30, email: "alice@example.com" },
    { id: 2, name: "Bob", age: 25, email: "bob@example.com" },
    { id: 3, name: "Charlie", age: 35, email: "charlie@example.com" }
  ];

  useEffect(() => {
    // Simulate API call with mock data
    setTimeout(() => {
      try {
        // Simulate success response
        setUsers(mockData);
        setLoading(false);
      } catch (err) {
        // Simulate error response
        setError('Failed to fetch data');
        setLoading(false);
      }
    }, 1000); // Simulate a delay for the loading state
  }, []);
*/
  
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
    return (
      <div className="min-h-screen flex items-center justify-center bg-red-100">
        <div className="p-4 bg-red-500 text-white rounded-lg shadow-md">
          <h2 className="text-lg font-semibold">Error</h2>
          <p>{`Error: ${error}`}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-100 p-8 flex flex-col items-center">
      <h1 className="text-4xl font-extrabold text-gray-800 mb-8">Data from API</h1>
      {loading ? (
        <div className="flex items-center justify-center space-x-2">
          <div className="w-8 h-8 border-4 border-blue-500 border-t-transparent border-solid rounded-full animate-spin"></div>
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          {users.map(user => (
            <div
              className="bg-white shadow-md rounded-lg p-6 border border-gray-200"
              key={user.id}
            >
              <ul className="space-y-4">
                <li>
                  <strong className="text-gray-700">Name:</strong> {user.name}
                </li>
                <li>
                  <strong className="text-gray-700">Age:</strong> {user.age}
                </li>
                <li>
                  <strong className="text-gray-700">Email:</strong> {user.email}
                </li>
              </ul>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

export default App;
