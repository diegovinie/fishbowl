<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { products as productsApi } from '@/services/api';

interface Product {
    id: number,
    name: string,
    price: number,
    available: boolean,
}

const products = ref<Product[]>([]);

function fetchProducts() {
    return productsApi.list()
        .then(({ data: { data } }) => {
          products.value = data;
        })
}

onMounted(() => {
    fetchProducts();
})
</script>

<template>
    <main class="flex">
      <div class="m-auto">
        <div>
          <button class="btn" @click="fetchProducts" >Refresh</button>
          <button class="btn m-4">Add product</button>
        </div>
        <table>
          <tr v-for="product in products" :key="product.id">
            <td class="pr-4 py-2">{{ product.id }}</td>
            <td class="pr-4 py-2">{{ product.name }}</td>
            <td class="pr-4 py-2 text-right">{{ product.price }}</td>
            <td class="pr-4 py-2">{{ product.available ? ':)' : 'X(' }}</td>
          </tr>
        </table>
      </div>
    </main>
</template>