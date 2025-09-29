use crate::prelude::*;
use deadpool_postgres::Pool;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query, SimpleExpr};
use serde_json;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use strum::{Display, EnumIter, IntoEnumIterator};
use tokio_postgres::Row;
use tokio_postgres::types::FromSql;
use postgres_types::ToSql;

pub use self::{user::UserStore};

pub mod user;

type InsertValues = Vec<SimpleExpr>;

trait FromRow: Sized {
    fn from_row(row: &Row, table_prefix: Option<&str>) -> Self;
}

trait RowValue {
    fn value<'a, T, V>(&'a self, name: V, table_prefix: Option<&str>) -> T
    where
        V: Iden,
        T: FromSql<'a>;
}

impl RowValue for Row {
    fn value<'a, T, V>(&'a self, name: V, table_prefix: Option<&str>) -> T
    where
        V: Iden,
        T: FromSql<'a>,
    {
        let idx = if let Some(v) = table_prefix {
            format!("{}.{}", v, name.to_string())
        } else {
            name.to_string()
        };

        self.get(idx.as_str())
    }
}