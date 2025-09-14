use std::sync::Arc;

use anyhow::Result;

use crate::domain::{entities::slots::SlotEntity, repositories::slot_viewing::SlotViewingRepository};


pub struct SlotViewingUseCase<T>
where
    T: SlotViewingRepository,
{
    slot_viewing_repository: Arc<T>,
}

impl<T> SlotViewingUseCase<T>
where
    T: SlotViewingRepository + Send + Sync,
{
    pub fn new(slot_viewing_repository: Arc<T>) -> Self {
        Self {
            slot_viewing_repository,
        }
    }

    async fn get_slots(&self) -> Result<Vec<SlotEntity>> {
        let schedules = self.slot_viewing_repository.get_slots().await?;
        Ok(schedules)
    }

    async fn get_doctor_slots(&self, doctor_id: i32) -> Result<Vec<SlotEntity>> {
        let schedules = self.slot_viewing_repository.get_doctor_slots(doctor_id).await?;
        Ok(schedules)
    }

}