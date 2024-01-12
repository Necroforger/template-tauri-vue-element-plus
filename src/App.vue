<script setup>
import Form from './components/Form.vue'
import { onMounted } from 'vue'
import { window as win } from '@tauri-apps/api'

onMounted(async () => {
  // Give the application some time to load so that it doesn't flash white.
  addEventListener("DOMContentLoaded", async () => {
    setTimeout(async () => {
      await win.appWindow.show();
      await win.appWindow
    }, 120);
  })

  // Prevent the page from being refreshed with f5 and ctrl + r.
  addEventListener("keydown", (e) => {
    const k =  e.key.toLowerCase()
    if (
      (e.ctrlKey || e.metaKey) && k == 'r'
      || k == 'f5'
    ) {
      e.preventDefault();
    }
  })

  // Prevent the right click context menu from appearing.
  addEventListener("contextmenu", (e) => {
    e.preventDefault();
  })
})
</script>

<template>
  <div class="container">
    <Form></Form>
  </div>
</template>

<style scoped>
</style>
