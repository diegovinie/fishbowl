<script setup lang="ts">
import { profile } from '@/store';
import { auth } from '@/services/api';

const logout = async () => {
    await auth.logout();

    profile.email = '';
}
</script>

<template>
    <header class="p-4 bg-stone-400 flex">
        <div class="my-auto flex gap-2">
            <RouterLink to="login">login</RouterLink>
            <RouterLink to="home">home</RouterLink>
            <RouterLink to="products">products</RouterLink>
        </div>
        <div class="text-right ml-auto my-auto mr-4">
            {{ profile.email ?? 'nobody' }}
        </div>
        <button @click="logout" class="btn">Logout</button>
    </header>
</template>