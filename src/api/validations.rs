use salvo::http::form::FormData;
use chrono::NaiveDateTime;
use crate::api::utils::hash_password;

use super::errors::{ApiResult, ApiError};
use super::utils::formatters::optional_date::FORMAT;

pub trait Validator {
    fn string(&self, key: &str) -> ApiResult<String>;

    fn optional_string(&self, key: &str) -> ApiResult<Option<String>>;

    fn integer(&self, key: &str) -> ApiResult<i32> {
        self.string(key)?
            .parse()
            .map_err(|error| ApiError::ParseInt(error, key.to_string()))
    }

    fn float(&self, key: &str) -> ApiResult<f32> {
         self.string(key)?
            .parse::<f32>()
            .map_err(|error| ApiError::ParseFloat(error, key.to_string()))
    }


    fn boolean(&self, key: &str) -> ApiResult<bool> {
        self.string(key)?
            .parse()
            .map_err(|e| ApiError::ParseBool(e, key.to_string()))
    }

    fn optional_boolean(&self, key: &str) -> ApiResult<Option<bool>> {
        match self.optional_string(key)? {
            None => Ok(None),
            Some(value) => {
                let boolean: bool = value.parse()
                    .map_err(|e| ApiError::ParseBool(e, key.to_string()))?;

                Ok(Some(boolean))
            },
        }
    }

    fn optional_date(&self, key: &str) -> ApiResult<Option<NaiveDateTime>> {
        match self.optional_string(key)? {
            None => Ok(None),
            Some(s) => {
                let date = NaiveDateTime::parse_from_str(&s, FORMAT)?;
                Ok(Some(date))
            },
        }
    }

    fn password(&self, key: &str) -> ApiResult<Vec<u8>> {
        let pwd = self.string(key)?;
        let hashed = hash_password(&pwd);

        Ok(hashed)
    }
}

pub struct FormValidator<'a>(pub &'a FormData);

impl Validator for FormValidator<'_> {
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
}

#[cfg(test)]
mod tests {
    use chrono::Datelike;
    use salvo::http::form::FormData;
    use super::{Validator, FormValidator};

    fn form_data(fields: &[(&str, &str)]) -> FormData {
        let mut form_data = FormData::new();

        for (k, v) in fields {
            form_data.fields.insert(k.to_string(), v.to_string());
        }

        form_data
    }

    #[test]
    fn validate_form_data() {
        let form_data = form_data(&[
            ("name", "Britney Swift"),
            ("age", "54"),
            ("salary", "2040000.05"),
            ("expiration", "2025-02-22 13:00:00"),
            ("password", "anything"),
            ("active_career", "true"),
            ("married", "false"),
            ("cinema", "false"),
        ]);

        let validator = FormValidator(&form_data);

        assert_eq!(validator.string("name").unwrap(), "Britney Swift");
        assert_eq!(validator.optional_string("nickname").unwrap(), None);
        assert_eq!(validator.integer("age").unwrap(), 54);
        assert_eq!(validator.float("salary").unwrap(), 2040000.05);
        assert_eq!(validator.optional_date("expiration").unwrap().unwrap().month(), 2);
        assert_eq!(validator.optional_date("expedition").unwrap(), None);
        assert!(validator.password("password").unwrap().len() > 0);
        assert_eq!(validator.boolean("active_career").unwrap(), true);
        assert_eq!(validator.boolean("married").unwrap(), false);
        assert_eq!(validator.optional_boolean("children").unwrap(), None);
        assert_eq!(validator.optional_boolean("cinema").unwrap(), Some(false));
    }
}