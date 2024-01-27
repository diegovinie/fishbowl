# Fishbowl

## API

### Routes
```
GET     (PA)    /admin/users
POST    (PA)    /admin/populate/users
POST    (PA)    /admin/populate/products
GET             /products
POST            /products
GET             /products/<id>
PUT             /products/<id>
DELETE          /products/<id>
GET     (P)     /wishlists
POST    (P)     /wishlists
GET     (P)     /wishlists/user
GET     (P)     /wishlists/<id>
PUT     (P)     /wishlists/<id>
DELETE  (P)     /wishlists/<id>
GET     (P)     /wishlists/<wishlist_id>/wishes
POST    (P)     /wishlists/<wishlist_id>/wishes
GET     (P)     /wishlists/<wishlist_id>/wishes/<id>
DELETE  (P)     /wishlists/<wishlist_id>/wishes/<id>

```

### Routes TODO
```
GET     (P)     /wishes/<id>
PUT     (P)     /wishes/<id>
DELETE  (P)     /wishes/<id>
GET     (P)     /wishes/<wish_id>/sponsors
POST    (P)     /wishes/<wish_id>/sponsors
GET     (P)     /me
GET     (P)     /me/wishlists
GET     (P)     /me/sponsors
```

## TODO
1. Add salt to password
1. Implement refresh token
1. Populate with meaningful wishlists
1. Sponsor's crud