<script setup lang="ts">
import AddWishForm from '@/components/AddWishForm.vue'
import type { ListedWishlist, Wish } from '@/interfaces'
import * as api from '@/services/api'
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

interface DetailedWishlist extends ListedWishlist {
  description: string
  published: boolean
  wishes: Wish[]
}

const details = ref<DetailedWishlist>()

const formShown = ref(false)

const wishlistId = Number(route.params.id as string)

const fetchWishlistDetails = () => {
  api.wishlists.showDetailed(wishlistId).then(({ data: { data } }) => {
    details.value = data
  })
}

const toggleFormShown = () => {
  formShown.value = !formShown.value
}

const deleteWish = (id: number) => {
  api.wishes.delete(wishlistId, id).then(fetchWishlistDetails)
}

const onWishAdded = () => {
  fetchWishlistDetails()
}

onMounted(() => {
  fetchWishlistDetails()
})
</script>

<template>
  <main class="flex p-4 flex-col">
    <div class="m-auto">
      <ul v-if="details">
        <h1 class="text-lg mb-4">{{ details.title }}</h1>
        <li>{{ details.description }}</li>
        <li>Published: {{ details.published }}</li>
        <li>
          <div>wishes:</div>
          <table class="bg-base-light">
            <tr class="bg-base text-center">
              <th class="px-4 py-1">pending</th>
              <th>name</th>
              <th>price</th>
              <th>actions</th>
            </tr>
            <tr v-for="wish in details.wishes" :key="wish.id">
              <td>{{ wish.pending }}</td>
              <td class="px-4">{{ wish.product.name }}</td>
              <td class="text-right py-2">{{ wish.product.price }}</td>
              <td class="px-4">
                <button class="btn btn-sm bg-primary mr-4">sponsors</button>
                <button @click="deleteWish(wish.id)" class="btn btn-sm bg-tertiary">delete</button>
              </td>
            </tr>
          </table>
          <button @click="toggleFormShown" class="btn mt-4 bg-primary">
            {{ formShown ? 'close form' : 'Add product' }}
          </button>
        </li>
      </ul>
    </div>
    <section v-if="formShown" class="mx-auto mt-10">
      <AddWishForm :wishlist-id="wishlistId" @wish-added="onWishAdded" />
    </section>
  </main>
</template>
