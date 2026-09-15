<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const name = ref("");
const greeting = ref("");
const loading = ref(false);

async function greet() {
  if (!name.value.trim()) return;
  loading.value = true;
  try {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greeting.value = await invoke<string>("greet", { name: name.value });
  } catch (error) {
    greeting.value = `调用失败：${String(error)}`;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input v-model="name" class="greet-input" placeholder="Enter a name..." />
    <button type="submit" :disabled="loading">
      {{ loading ? "调用中…" : "Greet" }}
    </button>
  </form>
  <p>{{ greeting }}</p>
</template>
