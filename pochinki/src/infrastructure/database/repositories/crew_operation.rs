use anyhow::Result;
use std::sync::Arc;
use diesel::prelude::*;
use diesel::{insert_into, delete};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;

use crate::infrastructure::database::postgresql_connection::PgPoolSquad;
use crate::domain::repositories::crew_operation::CrewOperationRepository;
use crate::domain::entities::crew_membership::CrewMemberShips;
use crate::infrastructure::database::schema::crew_memberships;

pub struct CrewParticipationPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl CrewParticipationPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait::async_trait]
impl CrewOperationRepository for CrewParticipationPostgres {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;
        insert_into(crew_memberships::table)
            .values(crew_memberships)
            .execute(&mut connection)?;

        Ok(())
    }

    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()> {
        let mut connection = Arc::clone(&self.db_pool).get()?;

        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(&mut connection)?;

        Ok(())
    }

    fn for_insert_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMemberShips,
    ) -> Result<()> {
        insert_into(crew_memberships::table)
            .values(crew_memberships)
            .execute(conn)?;

        Ok(())
    }

    fn for_delete_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMemberShips,
    ) -> Result<()> {
        delete(crew_memberships::table)
            .filter(crew_memberships::brawler_id.eq(crew_memberships.brawler_id))
            .filter(crew_memberships::mission_id.eq(crew_memberships.mission_id))
            .execute(conn)?;

        Ok(())
    }
}
