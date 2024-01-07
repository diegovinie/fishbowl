
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