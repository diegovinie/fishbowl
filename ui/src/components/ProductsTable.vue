<script setup lang="ts">
import type { ListedProduct } from '@/interfaces'

interface ProductTableProps {
  products: ListedProduct[]
  selectedId?: number
}

defineProps<ProductTableProps>()
</script>

<template>
  <table class="bg-base-light">
    <tr
      :class="product.id === selectedId ? 'bg-primary-light' : ''"
      v-for="product in products"
      :key="product.id"
      @click="$emit('productClicked', product)"
    >
      <td class="pr-4 py-2">{{ product.id }}</td>
      <td class="pr-4 py-2">{{ product.name }}</td>
      <td class="pr-4 py-2 text-right">{{ product.price }}</td>
      <td class="pr-4 py-2">{{ product.available ? ':)' : 'X(' }}</td>
    </tr>
  </table>
</template>
