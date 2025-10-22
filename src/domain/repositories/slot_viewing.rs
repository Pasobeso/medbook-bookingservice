use anyhow::Result;

use crate::domain::entities::slots::SlotEntity;

pub trait SlotViewingRepository {
    async fn get_slots(&self) -> Result<Vec<SlotEntity>>;
    async fn get_doctor_slots(&self, doctor_id: i32) -> Result<Vec<SlotEntity>>;
}
