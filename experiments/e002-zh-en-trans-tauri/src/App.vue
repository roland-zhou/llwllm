<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const text = ref("");
const translationResult = ref("");
const error = ref("");
const isLoading = ref(false);

function handleKeydown(event: KeyboardEvent) {
  // Check for Cmd+Enter (Mac) or Ctrl+Enter (Windows/Linux)
  if ((event.metaKey || event.ctrlKey) && event.key === 'Enter') {
    event.preventDefault();
    translate();
  }
}

async function translate() {
  if (!text.value.trim()) {
    error.value = "Please enter some text to translate";
    return;
  }

  error.value = "";
  isLoading.value = true;
  try {
    translationResult.value = await invoke("translate", { text: text.value });
  } catch (e: any) {
    error.value = e.message || "Translation failed";
    translationResult.value = "";
  } finally {
    isLoading.value = false;
  }
}
</script>

<template>
  <main class="container">
    <form class="row" @submit.prevent="translate">
      <textarea 
        id="translate-input" 
        v-model="text" 
        placeholder="Enter text to translate..." 
        :disabled="isLoading"
        rows="5"
        @keydown="handleKeydown"
      ></textarea>
      <button type="submit" :disabled="isLoading">
        {{ isLoading ? 'Translating...' : 'Translate' }}
      </button>
    </form>
    <div class="result-container">
      <p v-if="error" class="error">{{ error }}</p>
      <p v-else-if="translationResult" class="result">{{ translationResult }}</p>
    </div>
  </main>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 2vh;
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
}


.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
  width: 100%;
  padding: 0 1rem;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button,
textarea {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button,
textarea {
  outline: none;
}

#translate-input {
  margin-right: 5px;
  flex: 1;
  resize: vertical;
  min-height: 100px;
  padding: 8px;
  font-family: inherit;
  font-size: inherit;
  border-radius: 4px;
  border: 1px solid #ccc;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button,
  textarea {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

.error {
  color: #dc2626;
  margin-top: 1rem;
}

.result-container {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  padding: 0 1rem;
  width: 100%;
}

.result {
  margin-top: 1rem;
  white-space: pre-wrap;
  text-align: left;
  padding: 1rem;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
}
</style>