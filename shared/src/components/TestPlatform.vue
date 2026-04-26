<template>
  <div class="test-platform">
    <h2>PlatformBridge Integration Test</h2>
    <div>
      <p>Environment: {{ isDesktop ? 'Desktop (Tauri)' : 'Web (WASM)' }}</p>
      <p>Bridge initialized: {{ isInitialized }}</p>
      <p>Hardware profile: {{ hardwareProfile }}</p>
    </div>
    <div>
      <button @click="testHealthCheck" :disabled="testing">
        {{ testing ? 'Testing...' : 'Test Health Check' }}
      </button>
      <button @click="testInvoke" :disabled="testing">
        Test Invoke
      </button>
      <button @click="testInvokeWithResult" :disabled="testing">
        Test InvokeWithResult
      </button>
    </div>
    <div v-if="result">
      <h3>Result:</h3>
      <pre>{{ result }}</pre>
    </div>
    <div v-if="error">
      <h3 style="color: red">Error:</h3>
      <pre>{{ error }}</pre>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { usePlatform } from '../composables/usePlatform'

const { isDesktop, isInitialized, invoke, invokeWithResult, detectHardware } = usePlatform()

const testing = ref(false)
const result = ref<any>(null)
const error = ref<string | null>(null)

const hardwareProfile = computed(() => detectHardware())

const testHealthCheck = async () => {
  testing.value = true
  result.value = null
  error.value = null
  try {
    const response = await invoke<string>('health_check')
    result.value = response
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err)
  } finally {
    testing.value = false
  }
}

const testInvoke = async () => {
  testing.value = true
  result.value = null
  error.value = null
  try {
    // Test a simple command that returns a string
    const response = await invoke<string>('get_app_version', {})
    result.value = response
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err)
  } finally {
    testing.value = false
  }
}

const testInvokeWithResult = async () => {
  testing.value = true
  result.value = null
  error.value = null
  try {
    const response = await invokeWithResult<string>('health_check')
    result.value = response
  } catch (err) {
    error.value = err instanceof Error ? err.message : String(err)
  } finally {
    testing.value = false
  }
}
</script>

<style scoped>
.test-platform {
  padding: 1rem;
  border: 1px solid #ccc;
  border-radius: 8px;
  margin: 1rem 0;
}
button {
  margin-right: 0.5rem;
  padding: 0.5rem 1rem;
  background: #007acc;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
button:disabled {
  background: #ccc;
  cursor: not-allowed;
}
pre {
  background: #f5f5f5;
  padding: 0.5rem;
  border-radius: 4px;
  overflow: auto;
}
</style>