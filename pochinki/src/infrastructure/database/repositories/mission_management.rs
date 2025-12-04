use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use diesel::prelude::*;
use diesel::insert_into;

use crate::domain::entities;
use crate::infrastructure::database::schema::missions;
use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::domain::repositories::mission_management::MissionManagementRepository;
use crate::domain::entities::missions::{AddMissionEntity, EditMissionEntity};
use crate::domain::value_objects::mission_statuses::MissionStatuses;

pub struct MisssionManagementPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl MisssionManagementPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl MissionManagementRepository for MisssionManagementPostgres {
    async fn add(&self, add_mission_entity: AddMissionEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result: i32 = insert_into(missions::table)
            .values(&add_mission_entity)
            .returning(missions::id)
            .get_result(&mut connection)?;

        Ok(result)
    }

    async fn edit(&self, mission_id: i32, edit_mission_entity: EditMissionEntity) -> Result<i32> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        let result = diesel::update(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .set(&edit_mission_entity)
            .returning(missions::id)
            .get_result::<i32>(&mut connection)?;

        Ok(result)
    }

    async fn remove(&self, mission_id: i32, chief_id: i32) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        diesel::delete(missions::table)
            .filter(missions::id.eq(mission_id))
            .filter(missions::status.eq(MissionStatuses::Open.to_string()))
            .filter(missions::chief_id.eq(chief_id))
            .execute(&mut connection)?;

        Ok(())
    }
}
