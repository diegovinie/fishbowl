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
        .then(({ data: { data } }) => data);
}

onMounted(async () => {
    products.value = await fetchProducts();
})
</script>

<template>
    <main>
        <ul>
            <li v-for="product in products" :key="product.id">
                <ul>
                    <li>{{ product.id }}</li>
                    <li>{{ product.name }}</li>
                    <li>{{ product.price }}</li>
                    <li>{{ product.available ? ':)' : 'X(' }}</li>
                </ul>
            </li>
        </ul>

        <button @click="fetchProducts">get</button>
    </main>
</template>