use axum::async_trait;

use crate::domain::entities::slots::SlotEntity;

#[async_trait]
pub trait SlotViewingRepository {
    async fn get_slots(&self) -> anyhow::Result<SlotEntity>;
    async fn get_doctor_slots(&self, doctor_id: i32) -> anyhow::Result<Vec<SlotEntity>>;
}