<script setup lang="ts">
import { useRoute } from 'vue-router';
import * as api from '@/services/api';
import { onMounted, ref } from 'vue';
import type { ListedWishlist, Wish } from '@/interfaces';
import AddWishForm from '@/components/AddWishForm.vue';

const route = useRoute();

interface DetailedWishlist extends ListedWishlist {
    description: string;
    published: boolean;
    wishes: Wish[]
}

const details = ref<DetailedWishlist>();

const wishlistId = Number(route.params.id as string);

const fetchWishlistDetails = () => {
    api.wishlists.showDetailed(wishlistId)
        .then(({ data: { data }}) => {
            details.value = data;
        })
}

const deleteWish = (id: number) => {
  api.wishes.delete(wishlistId, id)
    .then(fetchWishlistDetails)
}

const onWishAdded = () => {
  fetchWishlistDetails();
}

onMounted(() => {
    fetchWishlistDetails();
});

</script>

<template>
  <main class="flex p-4 flex-col">
    <div class="m-auto">
      <ul v-if="details">
        <li>{{ details.title }}</li>
        <li>{{ details.description }}</li>
        <li>{{ details.published }}</li>
        <li>
          <div>wishes:</div>
          <table>
            <tr v-for="wish in details.wishes" :key="wish.id">
              <td>{{ wish.pending }}</td>
              <td class="px-4">{{ wish.product.name }}</td>
              <td class="text-right">{{ wish.product.price }}</td>
              <td @click="deleteWish(wish.id)" class="px-4">x</td>
            </tr>
          </table>
          <button class="btn mt-4">Add product</button>
        </li>
      </ul>
    </div>
    <section class="mx-auto mt-10">
      <AddWishForm :wishlist-id="wishlistId" @wish-added="onWishAdded" />
    </section>
  </main>
</template>