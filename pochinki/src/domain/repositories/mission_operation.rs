use anyhow::Result;
use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

#[async_trait]
#[cfg_attr(test, mockall::automock)]
pub trait MissionOperationRepository {
    async fn in_progress(&self, mission_id: i32, chief_id: i32) -> Result<i32>;
    async fn to_completed(&self, mission_id: i32, chief_id: i32) -> Result<i32>;
    async fn to_failed(&self, mission_id: i32, chief_id: i32) -> Result<i32>;
}
