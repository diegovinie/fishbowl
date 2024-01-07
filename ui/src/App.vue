<script setup lang="ts">
import TheHeader from '@/components/TheHeader.vue';
import { onMounted } from 'vue';
import { loadAuthToken, setAuthInterceptor } from './services/api';
import { useRouter } from 'vue-router';

const router = useRouter();

const onUnauthorized = () => {
  router.push('/login');
}

onMounted(() => {
  loadAuthToken();

  setAuthInterceptor(onUnauthorized);
})
</script>

<template>
  <TheHeader />

  <RouterView />
</template>

<style scoped>

</style>
