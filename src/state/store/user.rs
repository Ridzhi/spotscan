use super::*;

#[derive(Iden, EnumIter)]
#[iden = "user"]
enum UserIden {
    Table,
    Id,
    TgUserId,
    TgUserAccessHash,
    Settings,
    CreatedAt,
}

impl Into<InsertValues> for User {
    fn into(self) -> InsertValues {
        vec![
            self.tg_user_id.into(),
            self.tg_user_access_hash.into(),
            serde_json::to_value(self.settings)
                .expect("impl Into<InsertValues> for User: settings key")
                .into(),
            {
                let v: time::PrimitiveDateTime = self.created_at.into();
                v.into()
            },
        ]
    }
}

impl FromRow for User {
    fn from_row(row: &Row, table_prefix: Option<&str>) -> Self {
        Self {
            id: row.value(UserIden::Id, table_prefix),
            tg_user_id: row.value(UserIden::TgUserId, table_prefix),
            tg_user_access_hash: row.value(UserIden::TgUserAccessHash, table_prefix),
            settings: {
                let v: serde_json::Value = row.value(UserIden::Settings, table_prefix);
                serde_json::from_value(v).expect("impl FromRow for User: settings key")
            },
            created_at: UtcDateTime(row.value(UserIden::CreatedAt, table_prefix)),
        }
    }
}

pub struct UserStore {
    pg: Arc<Pool>,
}

impl UserStore {
    pub fn new(pg: Arc<Pool>) -> Self {
        Self { pg }
    }

    pub async fn save(&self, v: User) -> Result<User> {
        let sql = Query::insert()
            .into_table(UserIden::Table)
            .columns(UserIden::iter().skip(2))
            .values(<User as Into<InsertValues>>::into(v))?
            .returning_all()
            .to_owned()
            .to_string(PostgresQueryBuilder);

        let row = self.pg.get().await?.query_one(&sql, &[]).await?;

        Ok(User::from_row(&row, None))
    }

    pub async fn update(&self, v: User) -> Result<User> {
        let id = v.id;
        let sql = Query::update()
            .table(UserIden::Table)
            .values(
                UserIden::iter()
                    .skip(2)
                    .zip(<User as Into<InsertValues>>::into(v).into_iter())
                    .collect::<Vec<(UserIden, SimpleExpr)>>()
            )
            .and_where(Expr::col(UserIden::Id).eq(id))
            .returning_all()
            .to_owned()
            .to_string(PostgresQueryBuilder);

        let row = self.pg.get().await?.query_one(&sql, &[]).await?;

        Ok(User::from_row(&row, None))
    }

    pub async fn find(&self, tg_user_id: TgUserId) -> Result<Option<User>> {
        self.find_one(vec![UserOption::TgUserId(tg_user_id)]).await
    }

    pub async fn find_one(&self, ops: UserOptions) -> Result<Option<User>> {
        Ok(self.find_many(ops).await?.first().cloned())
    }

    pub async fn find_many(&self, ops: UserOptions) -> Result<Vec<User>> {
        let mut q = Query::select()
            .from(UserIden::Table)
            .columns(UserIden::iter().skip(1))
            .to_owned();

        for o in ops {
            q.and_where(match o {
                UserOption::TgUserId(v) => Expr::col(UserIden::TgUserId).eq(v),
            });
        }

        let sql = q.to_string(PostgresQueryBuilder);
        let conn = self.pg.get().await?;

        let rows = conn.query(&sql, &[]).await?;

        Ok(rows
            .into_iter()
            .map(|row| User::from_row(&row, None))
            .collect())
    }
}