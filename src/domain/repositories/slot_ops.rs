use anyhow::Result;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::slots::{AddSlotEntity, EditSlotEntity};

pub trait SlotOpsRepository {
    async fn add(&self, add_slot_entity: AddSlotEntity) -> Result<Uuid>;
    async fn edit(
        &self,
        slot_id: Uuid,
        doctor_id: i32,
        edit_slot_entity: EditSlotEntity,
    ) -> Result<Uuid>;
    async fn remove(&self, slot_id: Uuid, doctor_id: i32) -> Result<()>;
}
