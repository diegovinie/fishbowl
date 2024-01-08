import type { ApiResponse, ListedProduct, Wish } from '@/interfaces';
import axios, { AxiosError, type AxiosResponse } from 'axios';

export const client = axios.create({
  baseURL: 'http://localhost:5800/api/v1/'
});



let authInterceptor: number | null;

export const setAuthInterceptor = (onUnauthorized: () => void) => {
  const onSuccess = (response: AxiosResponse) => response;

  const onError = (error: AxiosError) => {
    const status = error.response?.status;

    if ((status === 401 || status === 403)) {
      console.log('Auth failed', status);
      onUnauthorized();
    }

    return error;
  }

  authInterceptor = client.interceptors.response.use(onSuccess, onError);
}

export const ejectAuthInterceptor = () => {
  if (authInterceptor) {
    client.interceptors.response.eject(authInterceptor);
    authInterceptor = null;
  }
}

const setAuthToken = (token: string) => {
  client.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  localStorage.setItem('auth_token', token);
}

export const loadAuthToken = () => {
  const token = localStorage.getItem('auth_token');

  if (token) {
    client.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  }
}

const removeAuthToken = () => {
  delete client.defaults.headers.common.Authorization;

  localStorage.removeItem('auth_token');
}

export const auth = {
  login: (email: string, password: string) => {
    const formData = new FormData();
    formData.append('email', email);
    formData.append('password', password);

    return client.post('auth', formData)
      .then((res) => {
        setAuthToken(res.data);
      });
  },

  logout: () => {
    // future api logic
    removeAuthToken();
  }
}

export const products = {
  list: () => client.get<ApiResponse<ListedProduct[]>>('products'),
}

export const wishlists = {
  list: () => client.get('wishlists'),

  showDetailed: (id: number) => client.get(`wishlists/${id}?detailed=true`),
}

export const wishes = {
  addWish: (wishlistId: number, productId: number) => {
    const formData = new FormData();
    formData.append('wishlist_id', String(wishlistId));
    formData.append('product_id', String(productId));

    return client.post<FormData, ApiResponse<Wish>>(`wishlists/${wishlistId}/wishes`, formData);
  },

  delete: (wishlistId: number, wishId: number) => client.delete(`wishlists/${wishlistId}/wishes/${wishId}`),
}