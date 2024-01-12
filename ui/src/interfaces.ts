
export interface ListedProduct {
  id: number,
  name: string,
  price: number,
  available: boolean,
}

export interface Product {
  id: number,
  name: string,
  description: string,
  url: string,
  price: number,
  available: boolean,
}

export interface Wish {
  id: number;
  pending: boolean;
  product: Product
}

export interface ListedWishlist {
  id: number,
  title: string,
  date: string,
}

export interface ApiResponse<T> {
  data: T,
}

export interface Authentication {
  auth_token: string,
  user: User,
}

export interface User {
  id: number,
  name: string,
  email: string,
  role: string,
  active: boolean,
}