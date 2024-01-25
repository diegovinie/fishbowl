use salvo::prelude::*;
use crate::models::Role;
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