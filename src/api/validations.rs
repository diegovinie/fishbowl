use salvo::http::form::FormData;
use super::errors::{ApiResult, ApiError};

pub trait Validator {
    fn integer(&self, key: &str) -> ApiResult<i32>;

    fn float(&self, key: &str) -> ApiResult<f32>;

    fn string(&self, key: &str) -> ApiResult<String>;

    fn optional_string(&self, key: &str) -> ApiResult<Option<String>>;

    fn optional_boolean(&self, key: &str) -> ApiResult<Option<bool>>;
}

pub struct FormValidator<'a>(pub &'a FormData);

impl Validator for FormValidator<'_> {
    fn integer(&self, key: &str) -> ApiResult<i32> {
        self.0.fields.get(key)
            .ok_or(ApiError::FieldNotFound(key.to_string()))?
            .parse()
            .map_err(|error| ApiError::ParseInt(error, key.to_string()))
    }

    fn float(&self, key: &str) -> ApiResult<f32> {
        self.0.fields.get(key)
            .ok_or(ApiError::FieldNotFound(key.to_string()))?
            .parse::<f32>()
            .map_err(|error| ApiError::ParseFloat(error, key.to_string()))
    }

    fn string(&self, key: &str) -> ApiResult<String> {
        self.0.fields.get(key)
            .map(|n| n.to_string())
            .ok_or(ApiError::FieldNotFound(key.to_string()))
    }

    fn optional_string(&self, key: &str) -> ApiResult<Option<String>> {
        let value = self.0.fields.get(key)
            .map(|u| u.to_string());

        Ok(value)
    }

    fn optional_boolean(&self, key: &str) -> ApiResult<Option<bool>> {
        todo!()
    }

}