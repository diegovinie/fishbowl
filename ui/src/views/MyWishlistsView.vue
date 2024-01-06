<script setup lang="ts">
import type { ListedWishlist } from '@/interfaces';
import { ref, onMounted } from 'vue';
import { wishlists as wishlistsApi } from '@/services/api';
import WishlistsTable from '@/components/WishlistsTable.vue';

const wishlists = ref<ListedWishlist[]>([]);

const fetchWishlists = () => {
  wishlistsApi.list()
    .then(({ data: { data }}) => {
      wishlists.value = data;
    });
}

onMounted(() => {
  fetchWishlists();
});

</script>

<template>
  <main class="flex">
    <div class="m-auto">
      <div>
        <button class="btn" @click="fetchWishlists" >Refresh</button>
        <button class="btn m-4">Add wishlist</button>
      </div>
      <WishlistsTable :wishlists="wishlists" />
    </div>
  </main>
</template>