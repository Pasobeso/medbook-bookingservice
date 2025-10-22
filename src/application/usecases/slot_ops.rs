use std::sync::Arc;

use anyhow::Result;
use uuid::Uuid;

use crate::domain::{
    repositories::slot_ops::SlotOpsRepository,
    value_objects::slot_model::{AddSlotDto, EditSlotDto},
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

    pub async fn add(&self, doctor_id: i32, add_slot_dto: AddSlotDto) -> Result<Uuid> {
        let current_time = chrono::Utc::now().naive_utc();
        let add_slot_entity = add_slot_dto.to_entity(doctor_id, current_time);

        let slot_id = self.slot_ops_repository.add(add_slot_entity).await?;
        Ok(slot_id)
    }

    pub async fn edit(
        &self,
        slot_id: Uuid,
        doctor_id: i32,
        edit_slot_dto: EditSlotDto,
    ) -> Result<Uuid> {
        let current_time = chrono::Utc::now().naive_utc();
        let edit_slot_entity = edit_slot_dto.to_entity(current_time);

        let slot_id = self
            .slot_ops_repository
            .edit(slot_id, doctor_id, edit_slot_entity)
            .await?;
        Ok(slot_id)
    }

    pub async fn remove(&self, slot_id: Uuid, doctor_id: i32) -> Result<()> {
        self.slot_ops_repository.remove(slot_id, doctor_id).await?;

        Ok(())
    }
}
