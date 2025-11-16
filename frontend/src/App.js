import React, { useState, useEffect } from 'react';
import axios from 'axios';

function App() {
  const [status, setStatus] = useState('Loading...');
  const [aiHealth, setAiHealth] = useState(null);

  useEffect(() => {
    async function checkHealth() {
      try {
        const response = await axios.get('http://localhost:9000/health');
        setAiHealth(response.data);
        setStatus('All systems operational');
      } catch (error) {
        setStatus('AI Engine unavailable');
      }
    }
    checkHealth();
    const interval = setInterval(checkHealth, 5000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div style={styles.container}>
      <div style={styles.card}>
        <h1 style={styles.title}>Pepakura Next</h1>
        <p style={styles.subtitle}>3D Model to Papercraft Unfolding Engine</p>
        <div style={styles.status}>
          <p><strong>Status:</strong> {status}</p>
          {aiHealth && <p><strong>AI Engine:</strong> {aiHealth.status}</p>}
        </div>
      </div>
    </div>
  );
}

const styles = {
  container: { 
    display: 'flex', 
    justifyContent: 'center', 
    alignItems: 'center', 
    minHeight: '100vh',
    padding: '20px'
  },
  card: {
    backgroundColor: 'white',
    borderRadius: '10px',
    padding: '30px',
    boxShadow: '0 4px 6px rgba(0,0,0,0.1)',
    textAlign: 'center',
    maxWidth: '500px',
    width: '100%'
  },
  title: {
    fontSize: '2em',
    color: '#333',
    marginBottom: '10px'
  },
  subtitle: {
    fontSize: '1.2em',
    color: '#666',
    marginBottom: '20px'
  },
  status: {
    marginTop: '20px',
    textAlign: 'left'
  }
};

export default App;
