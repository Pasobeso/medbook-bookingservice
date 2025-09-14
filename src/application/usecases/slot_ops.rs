use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    repositories::slot_ops::SlotOpsRepository, value_objects::slot_model::AddSlotDto,
};

pub struct SlotOpsUseCase<T>
where
    T: SlotOpsRepository,
{
    slot_ops_repository: Arc<T>,
}

impl<T> SlotOpsUseCase<T>
where
    T: SlotOpsRepository + Send + Sync,
{
    pub fn new(slot_ops_repository: Arc<T>) -> Self {
        Self {
            slot_ops_repository,
        }
    }

    pub async fn add(&self, add_slot_dto: AddSlotDto, doctor_id: i32) -> Result<Uuid> {
        let current_time = chrono::Utc::now().naive_utc();
        let add_slot_entity = add_slot_dto.to_entity(doctor_id, current_time);

        let slot_id = self.slot_ops_repository.add(add_slot_entity).await?;
        Ok(slot_id)
    }

    pub async fn remove(&self, slot_id: Uuid, doctor_id: i32) -> Result<()> {
        self.slot_ops_repository.remove(slot_id, doctor_id).await?;

        Ok(())
    }
}
