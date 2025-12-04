use anyhow::Result;
use std::sync::Arc;
use diesel::prelude::*;

use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::domain::repositories::mission_viewing::MissionViewingRepository;
use crate::domain::entities::missions::MissionEntity;
use crate::domain::value_objects::mission_filter::MissionFilter;
use crate::infrastructure::database::schema::{missions, crew_memberships};

pub struct MissionViewingPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MissionViewingPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl MissionViewingRepository for MissionViewingPostgres {
    async fn view_detail(&self, mission_id: i32) -> Result<MissionEntity> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = missions::table
            .filter(missions::id.eq(mission_id))
            .filter(missions::deleted_at.is_null())
            .select(MissionEntity::as_select())
            .first::<MissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn get(&self, filter: &MissionFilter) -> Result<Vec<MissionEntity>> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let mut query = missions::table
            .filter(missions::deleted_at.is_null())
            .into_boxed();

        if let Some(status) = &filter.status {
            query = query.filter(missions::status.eq(status.to_string()));
        }

        if let Some(name) = &filter.name {
            query = query.filter(missions::name.ilike(format!("%{}%", name)));
        }

        let result = query
            .select(MissionEntity::as_select())
            .order_by(missions::created_at.desc())
            .load::<MissionEntity>(&mut conn)?;

        Ok(result)
    }

    async fn crew_counting(&self, mission_id: i32) -> Result<u32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        let result = crew_memberships::table
            .filter(crew_memberships::mission_id.eq(mission_id))
            .count()
            .get_result::<i64>(&mut conn)?;

        Ok(result as u32)
    }
}
