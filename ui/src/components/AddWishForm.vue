<script setup lang="ts">
import type { ListedProduct } from '@/interfaces';
import ProductsTable from './ProductsTable.vue';
import { onMounted, reactive, ref } from 'vue';
import * as api from '@/services/api';

interface AddWishFormProps {
    wishlistId: number;
}

const props = defineProps<AddWishFormProps>();

const selectedProduct = reactive<ListedProduct>({
    id: 1,
    name: '',
    price: 0,
    available: false,
})

const emit = defineEmits<{
    wishAdded: [value: number]
}>();

const products = ref<ListedProduct[]>();

const onProductSelected = (product: ListedProduct) => {
    selectedProduct.id = product.id;
    selectedProduct.name = product.name;
    selectedProduct.price = product.price;
}

const fetchProducts = () => {
    api.products.list()
        .then(({ data: { data }}) => {
            products.value = data.filter(p => p.available);
        });
}

const addWish = () => {
    api.wishes.addWish(props.wishlistId, selectedProduct.id)
        .then(res => {
            console.log(res);
            emit('wishAdded', 999);
        });
}

onMounted(() => {
    fetchProducts();
});

</script>

<template>
    <form @submit.prevent="addWish">
        <h2>Add product:</h2>
        <ProductsTable v-if="products" :products="products" @productClicked="onProductSelected" />
        <div class="">
            <input type="text" placeholder="name" v-model="selectedProduct.name">
        </div>
        <div class="">
            <input type="number" placeholder="price" v-model="selectedProduct.price">
        </div>
        <div>
            <button class="btn" type="submit">Add</button>
        </div>
    </form>
</template>