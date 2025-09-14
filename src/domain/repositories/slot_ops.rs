use anyhow::Result;
use axum::async_trait;
use uuid::Uuid;

use crate::domain::entities::slots::{AddSlotEntity};

#[async_trait]
pub trait SlotOpsRepository {
    async fn add(&self, add_slot_entity: AddSlotEntity) -> Result<Uuid>;
    async fn remove(&self, slot_id: Uuid, doctor_id: i32) -> Result<()>;
}