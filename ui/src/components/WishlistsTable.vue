<script setup lang="ts">
import type { ListedWishlist } from '@/interfaces';

interface WishlistsTableProps {
  wishlists: ListedWishlist[]
}

defineProps<WishlistsTableProps>();
</script>

<template>
  <table>
    <tr v-for="wishlist in wishlists" :key="wishlist.id" @click="$emit('rowClicked', wishlist.id)">
      <td class="pr-4 py-2">{{ wishlist.id }}</td>
      <td class="pr-4 py-2">{{ wishlist.title }}</td>
      <td class="pr-4 py-2">{{ wishlist.date }}</td>
    </tr>
  </table>
</template>
