<script setup lang="ts">
import GreetPanel from "./components/GreetPanel.vue";
import tauriLogo from "./assets/tauri.svg";
import viteLogo from "./assets/vite.svg";
import vueLogo from "./assets/vue.svg";
</script>

<template>
  <main class="container">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
      <a href="https://vite.dev" target="_blank">
        <img :src="viteLogo" class="logo vite" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img :src="tauriLogo" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org" target="_blank">
        <img :src="vueLogo" class="logo vue" alt="Vue logo" />
      </a>
    </div>

    <p>Click on the Tauri logo to learn more about the framework</p>

    <GreetPanel />
  </main>
</template>
