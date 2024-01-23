use salvo::http::form::FormData;
use super::errors::{ApiResult, ApiError};

pub struct FormValidator<'a>(pub &'a FormData);

impl FormValidator<'_> {
    pub fn float(&self, key: &str) -> ApiResult<f32> {
        self.0.fields.get(key)
            .ok_or(ApiError::FieldNotFound(key.to_string()))?
            .parse::<f32>()
            .map_err(|error| ApiError::ParseFloat(error, key.to_string()))
    }

    pub fn string(&self, key: &str) -> ApiResult<String> {
        self.0.fields.get(key)
            .map(|n| n.to_string())
            .ok_or(ApiError::FieldNotFound(key.to_string()))
    }

    pub fn optional_string(&self, key: &str) -> ApiResult<Option<String>> {
        let value = self.0.fields.get(key)
            .map(|u| u.to_string());

        Ok(value)
    }
}