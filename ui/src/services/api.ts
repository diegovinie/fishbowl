import axios from 'axios';

export const client = axios.create({
  baseURL: 'http://localhost:5800/api/v1/'
});

export const products = {
  list: () => client.get('products'),
}