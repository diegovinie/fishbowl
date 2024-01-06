<script setup lang="ts">
import TheHeader from '@/components/TheHeader.vue';
import { onMounted } from 'vue';
import { loadAuthToken } from './services/api';

onMounted(() => {
  loadAuthToken();
})
</script>

<template>
  <TheHeader />

  <RouterView />
</template>

<style scoped>

</style>
