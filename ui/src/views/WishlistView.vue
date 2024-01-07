<script setup lang="ts">
import { useRoute } from 'vue-router';
import { wishlists as api } from '@/services/api';
import { onMounted, ref } from 'vue';
import type { ListedWishlist, Wish } from '@/interfaces';

const route = useRoute();


interface DetailedWishlist extends ListedWishlist {
    description: string;
    published: boolean;
    wishes: Wish[]
}

const details = ref<DetailedWishlist>();

const fetchWishlistDetails = () => {
    const id = Number(route.params.id as string);

    api.showDetailed(id)
        .then(({ data: { data }}) => {
            console.log(data);
            details.value = data;
        })
}

onMounted(() => {
    fetchWishlistDetails();
});

</script>

<template>

    <main class="flex p-4">
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
                    </tr>
                  </table>
                  <button class="btn mt-4">Add product</button>
                </li>
            </ul>
        </div>
    </main>
</template>