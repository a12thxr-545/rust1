use anyhow::Result;
use std::sync::Arc;
use diesel::prelude::*;
use diesel::insert_into;

use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::domain::repositories::brawlers::BrawlerRepository;
use crate::domain::entities::brawlers::RegisterBrawlerEntity;
use crate::domain::entities::brawlers::BrawlerEntity;
use crate::infrastructure::database::schema::brawlers;

pub struct BrawlerPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl BrawlerPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl BrawlerRepository for BrawlerPostgres {
    async fn register(&self, register_brawler_entity: RegisterBrawlerEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = insert_into(brawlers::table)
            .values(&register_brawler_entity)
            .returning(brawlers::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn find_by_username(&self, username: String) -> Result<BrawlerEntity> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = brawlers::table
            .filter(brawlers::username.eq(username))
            .select(BrawlerEntity::as_select())
            .first::<BrawlerEntity>(&mut connection)?;

        Ok(result)
    }
}
