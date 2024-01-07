import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import ProductsView from '../views/ProductsView.vue'
import LoginView from '@/views/LoginView.vue'
import MyWishlistsView from '@/views/MyWishlistsView.vue'
import WishlistView from '@/views/WishlistView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/login',
      name: 'login',
      component: LoginView
    },
    {
      path: '/products',
      name: 'products',
      component: ProductsView
    },
    {
      path: '/my/wishlists/:id',
      name: 'detailed-wishlist',
      component: WishlistView
    },

    {
      path: '/my/wishlists',
      name: 'my-wishlists',
      component: MyWishlistsView
    }
  ]
})

export default router
