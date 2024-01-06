<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { products as productsApi } from '@/services/api';
import ProductsTable from '@/components/ProductsTable.vue';
import type { ListedProduct } from '@/interfaces';


const products = ref<ListedProduct[]>([]);

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
        <ProductsTable :products="products" />
      </div>
    </main>
</template>