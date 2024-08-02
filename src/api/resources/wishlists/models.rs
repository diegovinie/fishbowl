use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use salvo::http::form::FormData;
use chrono::NaiveDateTime;
use crate::api::errors::{ApiError, ApiResult};
use crate::api::validations::{FormValidator, Validator};
use crate::schema::wishlists;
use crate::api::resources::wishes::models::WishProduct;
use crate::api::resources::users::models::User;
use crate::models::{Composable, Mergeable};
use crate::api::utils::formatters::optional_date;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    PartialEq,
    AsChangeset,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = crate::schema::wishlists)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Wishlist {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    #[serde(with = "optional_date")]
    pub date: Option<NaiveDateTime>,
    pub user_id: i32,
    pub published: bool,
}

#[derive(Debug, Insertable, Deserialize)]
#[diesel(table_name = wishlists)]
pub struct NewWishlist {
    pub title: String,
    pub description: Option<String>,
    // #[serde(default)]
    #[serde(with = "optional_date")]
    pub date: Option<NaiveDateTime>,
    pub user_id: i32,
    pub published: bool,
}

impl Mergeable for Wishlist {
    fn merge(self, form_data: &FormData) -> ApiResult<Self> {
        let validator = FormValidator(&form_data);
        let mut updated = self.clone();

        if let Some(title) = validator.get("title") {
            if title.is_empty() {
                return Err(ApiError::BadRequestError(format!("`title` cannot be empty")));
            }
            
            updated.title = validator.string("title")?;
        }

        if let Some(_) = validator.get("description") {
            updated.description = validator.optional_string("description")?;
        }

        if let Some(_) = validator.get("date") {
            updated.date = validator.optional_date("date")?;
        }

        if let Some(_) = validator.get("user_id") {
            return Err(ApiError::NotAllowed(format!("Changing `user_id` not allowed")));
        }

        if let Some(_) = validator.get("published") {
            updated.published = validator.boolean("published")?;
        }

        Ok(updated)
    }
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::wishlists)]
pub struct ListedWishlist {
    pub id: i32,
    pub title: String,
    #[serde(with = "optional_date")]
    pub date: Option<NaiveDateTime>,
}

impl From<Wishlist> for ListedWishlist {
    fn from(wishlist: Wishlist) -> Self {
        let Wishlist { id, title, date, .. } = wishlist;

        Self { id, title, date }
    }
}

#[derive(Serialize)]
pub struct DetailedWishlist {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    #[serde(with = "optional_date")]
    pub date: Option<NaiveDateTime>,
    pub user_id: i32,
    pub published: bool,
    pub wishes: Vec<WishProduct>,
}

impl Composable<Wishlist, Vec<WishProduct>> for DetailedWishlist {
    fn compose(wishlist: Wishlist, wishes: Vec<WishProduct>) -> Self {
        let Wishlist { id, title, description, date,user_id, published } = wishlist;

        Self { id, title, description, date, user_id, published, wishes }
    }
}
