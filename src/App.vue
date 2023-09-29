<script setup lang="ts">
import HomeView from './views/HomeView.vue'
</script>

<template>
  <n-notification-provider>
    <n-message-provider>
      <HomeView />
    </n-message-provider>
  </n-notification-provider>
</template>

<style scoped>
@media (min-width: 1024px) {
}
</style>
