<template>
  <div class="ai-generator">
    <h2>AI 3D Model Generator</h2>
    <div class="input-section">
      <input 
        v-model="prompt" 
        placeholder="Describe your 3D model (e.g., 'cube', 'pyramid')" 
        @keyup.enter="generateModel"
      />
      <button @click="generateModel" :disabled="loading">
        {{ loading ? 'Generating...' : 'Generate 3D Model' }}
      </button>
    </div>
    
    <div v-if="result" class="result">
      <h3>Generated Model:</h3>
      <div class="model-info">
        <p><strong>ID:</strong> {{ result.id }}</p>
        <p><strong>Vertices:</strong> {{ result.vertices }}</p>
        <p><strong>Faces:</strong> {{ result.faces }}</p>
        <p><strong>Model Type:</strong> {{ result.type }}</p>
      </div>
    </div>
    
    <div v-if="error" class="error">
      Error: {{ error }}
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const prompt = ref('')
const loading = ref(false)
const result = ref(null)
const error = ref(null)

function generateModel() {
  if (!prompt.value.trim()) return
  
  loading.value = true
  result.value = null
  error.value = null
  
  try {
    // Mock response for testing
    setTimeout(() => {
      result.value = {
        id: Math.random().toString(36).substr(2, 9),
        vertices: Math.floor(Math.random() * 100) + 10,
        faces: Math.floor(Math.random() * 50) + 5,
        type: prompt.value.toLowerCase().includes('cube') ? 'Cube' : 
              prompt.value.toLowerCase().includes('pyramid') ? 'Pyramid' : 'Generic'
      }
      loading.value = false
    }, 1000)
  } catch (err) {
    error.value = err.toString()
    loading.value = false
  }
}
</script>

<style scoped>
.ai-generator {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
}

.input-section {
  margin: 20px 0;
}

.input-section input {
  width: 70%;
  padding: 10px;
  font-size: 16px;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.input-section button {
  width: 25%;
  margin-left: 5%;
  padding: 10px;
  font-size: 16px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.input-section button:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.result {
  margin-top: 20px;
  padding: 20px;
  background-color: #f5f5f5;
  border-radius: 4px;
}

.model-info p {
  margin: 5px 0;
}

.error {
  color: red;
  margin-top: 20px;
  padding: 10px;
  background-color: #ffe6e6;
  border-radius: 4px;
}
</style>
