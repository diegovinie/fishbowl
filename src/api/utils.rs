use std::{env, error::Error, sync::Arc};
use salvo::prelude::*;
use serde::de::DeserializeOwned;
use sha2::{Sha256, Digest};
use crate::{api::errors::{ApiResult, ApiError}, models::Role};
use  crate::database::contracts::DatabaseService;
use super::auth::JwtClaims;

pub fn get_user_id(depot: &Depot) -> Option<i32> {
    match depot.jwt_auth_data::<JwtClaims>() {
        None => None,
        Some(data) => Some(data.claims.id),
    }
}

pub fn admin(depot: &Depot) -> bool {
    match depot.jwt_auth_data::<JwtClaims>() {
        None => false,
        Some(data) => match data.claims.role {
            Role::Admin => true,
            _ => false,
        }
    }
}

pub mod pagination {
    use diesel::query_builder::{QueryFragment, AstPass, QueryId, Query};
    use diesel::{pg::Pg, sql_types::BigInt};
    use diesel::prelude::*;
    use serde::{Serialize, Deserialize};

    const DEFAULT_PER_PAGE: i64 = 10;

    #[derive(Serialize, Deserialize)]
    pub struct Pagination {
        pub page: i64,
        pub per_page: i64,
        pub entries: i64,
        pub total_pages: i64,
    }

    impl Pagination {
        pub fn new(page: i64, per_page: i64, entries: i64) -> Self {
            Self {
                page,
                per_page,
                entries,
                total_pages: entries / per_page + if entries  % per_page == 0 { 0 } else { 1 }
            }
        }
    }

    #[derive(Debug, Clone, Copy, QueryId)]
    pub struct Paginated<T> {
        pub query: T,
        pub page: i64,
        pub per_page: i64,
        pub offset: i64,
    }

    impl<T> Paginated<T> {
        pub fn per_page(self, per_page: i64) -> Self {
            Paginated {
                per_page,
                offset: (self.page - 1) * per_page,
                ..self
            }
        }
    }

    impl<T> QueryFragment<Pg> for Paginated<T>
    where
        T: QueryFragment<Pg>,
     {
        fn walk_ast<'b>(&'b self, mut pass: AstPass<'_, 'b, Pg>) -> QueryResult<()> {
            pass.push_sql("SELECT *, COUNT(*) OVER () FROM (");
            self.query.walk_ast(pass.reborrow())?;
            pass.push_sql(") as paged_query_with LIMIT ");
            pass.push_bind_param::<BigInt, _>(&self.per_page)?;
            pass.push_sql(" OFFSET ");
            pass.push_bind_param::<BigInt, _>(&self.offset)?;
            Ok(())
        }
    }

    impl<T: Query> Query for Paginated<T> {
        type SqlType = (T::SqlType, BigInt);
    }

    impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

    pub trait Paginate: Sized {
        fn paginate(self, page: i64) -> Paginated<Self>;
    }

    impl<T> Paginate for T {
        fn paginate(self, page: i64) -> Paginated<Self> {
            Paginated {
                query: self,
                per_page: DEFAULT_PER_PAGE,
                page,
                offset: (page - 1) * DEFAULT_PER_PAGE,
            }
        }
    }
}

pub fn compare_passwords(pwd: &[u8], candidate: &str) -> bool {
    let mut hasher = Sha256::new();

    hasher.update(candidate.as_bytes());

    let hashed = hasher.finalize();

    &hashed[..] == pwd
}

pub fn hash_password(pwd: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();

    hasher.update(pwd.as_bytes());

    let result = hasher.finalize();

    Vec::from(&result[..])
}

pub fn parse_csv<F: Into<T> + DeserializeOwned, T>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let current_dir = env::current_dir()?;
    let mut rdr = csv::Reader::from_path(current_dir.join(filename))?;

    let mut users: Vec<T> = vec![];

    for result in rdr.deserialize() {
        let user: F = result?;
        users.push(user.into())
    }

    Ok(users)    
}

pub fn get_db(depot: &Depot) -> ApiResult<&Arc<dyn DatabaseService>> {
    use crate::api::errors::InjectionError;

    let service = depot.obtain::<Arc<dyn DatabaseService>>()
        .map_err(|_| ApiError::Injection(InjectionError))?;

    Ok(service)
}

pub mod formatters {
    pub mod optional_date {
        use chrono::NaiveDateTime;
        use serde::{self, Deserialize, Serializer, Deserializer};

        pub const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";


        pub fn serialize<S: Serializer>(maybe_date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error> {
            match maybe_date {
                None => {
                    serializer.serialize_none()
                },
                Some(date) => {
                    let s = date.format(FORMAT).to_string();
                    serializer.serialize_str(&s)
                }
            }
        }

        pub fn deserialize<'a, D: Deserializer<'a>>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error> {

            let maybe: Option<String> = Option::deserialize(deserializer)?;

            match maybe {
                None => Ok(None),
                Some(null) if null == "null" => Ok(None),
                Some(s) => {
                    let date_time = NaiveDateTime::parse_from_str(&s, FORMAT)
                        .map_err(serde::de::Error::custom)?;

                    Ok(Some(date_time))
                },
            }
        }
    }
}
