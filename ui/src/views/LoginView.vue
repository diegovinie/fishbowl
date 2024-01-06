<script setup lang="ts">
import { ref } from 'vue';
import { auth } from '@/services/api';
import { useRouter } from 'vue-router';
import { profile } from '@/store';

const email = ref('');

const password = ref('');

const router = useRouter();

const onSubmit = async () => {
  await auth.login(email.value, password.value);

  profile.email = email.value;

  router.push('/products');
}

</script>

<template>
  <main class="flex">
    <div class="m-auto">
      <div>
        <h1>
          Login
        </h1>
      </div>
      <form @submit.prevent="onSubmit" class="bg-gray-200 p-4">
        <div class="m-4">
          <input v-model="email" type="text" placeholder="email">
        </div>
        <div class="m-4">
          <input v-model="password" type="password" placeholder="password">
        </div>
        <div class="mt-8 text-center">
          <button type="submit" class="btn">Submit</button>
        </div>
      </form>
    </div>
  </main>
</template>