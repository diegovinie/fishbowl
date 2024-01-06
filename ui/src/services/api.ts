import axios from 'axios';

export const client = axios.create({
  baseURL: 'http://localhost:5800/api/v1/'
});

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
  list: () => client.get('products'),
}

export const wishlists = {
  list: () => client.get('wishlists')
}